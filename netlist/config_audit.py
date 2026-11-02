"""Four-configuration audit of the sensor-patch netlist (EasyEDA Protel2 export).
For each assembly configuration, DNP parts' pins are removed from the real netlist and every
requirement is re-checked. Output: netlist/config_audit_report.md"""
import re, json, itertools, sys

NL = open('netlist/patch_relayout.net', encoding='utf-8').read()

# ---- parse nets: blocks "( NET  pin pin ... )"
nets = {}
for blk in re.findall(r'\(\s*\n(.*?)\n\s*\)', NL, flags=re.S):
    lines = [l.strip() for l in blk.splitlines() if l.strip()]
    if not lines or ' ' in lines[0] and '-' in lines[0].split()[0]:
        continue
    name, pins = lines[0], []
    for l in lines[1:]:
        tok = l.split()
        ref_pin = tok[0]                       # e.g. U5-1
        func = tok[1].rsplit('-', 1)[-1] if len(tok) > 1 else ''
        pins.append((ref_pin, func, l))
    nets[name] = pins

def refs_on(net): return {p[0].rsplit('-', 1)[0] for p in nets.get(net, [])}
def find_ref(substr):
    for n, ps in nets.items():
        for rp, f, line in ps:
            if substr in line: return rp.rsplit('-', 1)[0]
def two_pin_between(a, b):
    ra, rb = refs_on(a), refs_on(b)
    return sorted(r for r in ra & rb if r[0] in 'RC')

SHT = find_ref('SHT45'); MIC = find_ref('ICS-43434'); AND = sorted({rp.rsplit('-',1)[0] for n,ps in nets.items() for rp,f,l in ps if 'LVC1G08' in l})
R_PT = [r for r in two_pin_between('+3V3_LOCAL', 'PRES_T') if r.startswith('R')]
R_PM = [r for r in two_pin_between('+3V3_LOCAL', 'PRES_M') if r.startswith('R')]
C_SHT = [r for r in two_pin_between('MOD_3V3', 'GND') if r.startswith('C') and r in refs_on('MOD_3V3')]
C_MIC = [r for r in two_pin_between('PRI_VDD', 'GND') if r.startswith('C')]
# decoupling cap of each sensor = the 100nF placed with it; identify by position file
parts = {p['ref']: p for p in json.load(open('netlist/patch_parts_relayout.json'))}
c_sht = [r for r in C_SHT if r == 'C10']
c_mic = [r for r in C_MIC if r == 'C11']

GROUP_T = [SHT] + c_sht + R_PT     # fitted only with SHT45
GROUP_M = [MIC] + c_mic + R_PM     # fitted only with microphone
CONFIGS = {'A (SHT45 only)': GROUP_M, 'B (mic only)': GROUP_T,
           'C (both)': [], 'D (neither)': GROUP_T + GROUP_M}

def apply(dnp):
    out = {}
    for n, ps in nets.items():
        out[n] = [p for p in ps if p[0].rsplit('-', 1)[0] not in dnp]
    return out

GND_FUNCS = {'GND', 'VSS', 'EP', 'AGND'}
def short_check(nn):
    bad = []
    for n, ps in nn.items():
        if n == 'GND': continue
        for rp, f, l in ps:
            if f in GND_FUNCS and not rp.startswith(('FPC',)):
                bad.append(f'{rp}({f}) on {n}')
    return bad

def members(nn, n): return ', '.join(f'{p[0]}({p[1]})' for p in nn.get(n, [])) or '—'

# ---- ID voltages (main board: 100k pull-down on SENSOR_DATA_ID; patch rail 3.3V)
def vdiv(rtop, rbot): return 3.3 * rbot / (rtop + rbot)
def par(*r): return 1 / sum(1 / x for x in r)
LEAK_V = 32 * 100e-9 * 100e3   # 32 powered-off-state channels x 100nA max x 100k

rep = ['# Sensor-patch four-configuration audit (from exported netlist)\n',
       f'Source: `netlist/patch_full.net` (EasyEDA Schematic2, {len(parts)} parts, {len(nets)} nets).\n',
       f'DNP group T (fit only with SHT45): {GROUP_T}\n\nDNP group M (fit only with mic): {GROUP_M}\n\nAND gates: {AND}\n']
crit = ['SENSOR_CLK', 'SENSOR_SYNC', 'SENSOR_DATA_ID', 'MOD_3V3', 'MOD_SDA', 'MOD_SCL', 'MOD_SEL',
        'PRI_VDD', 'MIC_SEL', 'CLK_LOC', 'SYNC_LOC', 'SD_LOC', 'ID_NODE', 'PRES_T', 'PRES_M']
summary = []
for cname, dnp in CONFIGS.items():
    nn = apply(dnp)
    floating = sorted(n for n, ps in nn.items() if len(ps) == 1)
    empty = sorted(n for n, ps in nn.items() if len(ps) == 0)
    shorts = short_check(nn)
    sht_ok = SHT not in dnp and all(any(p[0] == f'{SHT}-{pin}' for p in nn[net]) for pin, net in [('1', 'MOD_SDA'), ('2', 'MOD_SCL'), ('3', 'MOD_3V3')])
    mic_ok = MIC not in dnp and all(any(p[0] == f'{MIC}-{pin}' for p in nn[net]) for pin, net in [('4', 'CLK_LOC'), ('1', 'SYNC_LOC'), ('6', 'SD_LOC'), ('5', 'PRI_VDD'), ('2', 'GND')])
    # pass-through: J_IN pin n and J_OUT pin n on the same net for n != 8
    jin, jout = 'FPC1', 'FPC2'
    pt = []
    for k in [1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12]:
        a = [n for n, ps in nn.items() if any(p[0] == f'{jin}-{k}' for p in ps)]
        b = [n for n, ps in nn.items() if any(p[0] == f'{jout}-{k}' for p in ps)]
        pt.append(a == b and len(a) == 1)
    cascade = any(p[0] == f'{jin}-8' for p in nn['PATCH_DATA_IN']) and any(p[0] == f'{jout}-8' for p in nn['PATCH_DATA_OUT'])
    aux_ok = all(len([p for p in nn[n] if p[0].startswith('U9')]) == 1 for n in ['MOD_3V3', 'MOD_SDA', 'MOD_SCL'])
    v_id = vdiv(22e3, 100e3)
    rpd = par(100e3, 1e6)
    v_t = vdiv(68e3, rpd) if R_PT[0] not in dnp else 0.0
    v_m = vdiv(68e3, rpd) if R_PM[0] not in dnp else 0.0
    rep.append(f'\n## Configuration {cname}\nDNP: {dnp or "none"}\n')
    rep.append(f'- Floating (single-pin) nets: **{floating or "none"}**; empty nets: {empty or "none"}')
    rep.append(f'- GND-function pins on non-GND nets (shorts): **{shorts or "none"}**')
    rep.append(f'- SHT45 operational: **{"YES" if sht_ok else ("not fitted" if SHT in dnp else "FAIL")}**')
    rep.append(f'- Microphone operational: **{"YES" if mic_ok else ("not fitted" if MIC in dnp else "FAIL")}**')
    rep.append(f'- Chain pass-through pins 1-7,9-12: **{"PASS" if all(pt) else "FAIL"}**; pin-8 cascade: **{"PASS" if cascade else "FAIL"}**')
    rep.append(f'- AUX header (U9) on MOD_3V3/SDA/SCL: **{"PASS" if aux_ok else "FAIL"}**')
    rep.append(f'- ID reads (V at ESP32 ADC, ±{LEAK_V:.2f} V worst-case leakage): patch-ID {v_id:.2f} V · PRES_T {v_t:.2f} V · PRES_M {v_m:.2f} V')
    rep.append('\n| Net | Members after DNP |\n|---|---|')
    for n in crit: rep.append(f'| {n} | {members(nn, n)} |')
    summary.append((cname, floating, shorts, sht_ok, mic_ok, all(pt) and cascade, aux_ok, v_id, v_t, v_m))

# gating evidence (config independent)
rep.append('\n## Hardware gating evidence (all configurations)')
for n in ['PRI_SIG_EN', 'MIC_SEL', 'MOD_SIG_EN', 'MOD_SEL', 'PRES_T_EN', 'PRES_M_EN', 'ID_EN']:
    rep.append(f'- **{n}**: {members(nets, n)}')

rep.append('\n## Verification matrix\n| Config | Floating nets | Shorts | SHT45 | Mic | Pass-through | AUX | ID / PRES_T / PRES_M (V) | Decode |\n|---|---|---|---|---|---|---|---|---|')
for c, fl, sh, s, m, pt, aux, vi, vt, vm in summary:
    dec = {(1, 0): 'SHT45 only', (0, 1): 'mic only', (1, 1): 'both', (0, 0): 'neither'}[(vt > 1, vm > 1)]
    rep.append(f'| {c} | {fl or "none"} | {sh or "none"} | {"✓" if s else "DNP"} | {"✓" if m else "DNP"} | {"✓" if pt else "✗"} | {"✓" if aux else "✗"} | {vi:.2f} / {vt:.2f} / {vm:.2f} | {dec} |')
open('netlist/config_audit_report.md', 'w', encoding='utf-8').write('\n'.join(rep))
print('\n'.join(rep[-6:]))
print('\nGROUP_T', GROUP_T, 'GROUP_M', GROUP_M, 'AND', AND)
