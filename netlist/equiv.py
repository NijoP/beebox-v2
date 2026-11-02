"""Prove a re-laid-out sheet is electrically identical to its golden snapshot.
Signature per part = (device name, sorted (pin number, net), sorted NC pins). Multisets must match."""
import json, sys, urllib.request, collections
BOARD = sys.argv[1]
DOC = {"main": "3d113e2c2213fddc", "patch": "df98848100b24fd4", "splitter": "86264b6bbddc15ff", "c_patch": "dc18354c68ca78d6", "c_split": "68d2d656cee68532"}[BOARD]
code = open('easyeda_scripts/snap.js').read()
code = code.replace('for(const [name,doc] of [["main","3d113e2c2213fddc"],["patch","df98848100b24fd4"]])', f'for(const [name,doc] of [["{BOARD}","{DOC}"]])')
r = json.loads(urllib.request.urlopen(urllib.request.Request("http://127.0.0.1:49620/execute", data=json.dumps({"code": code}).encode(), headers={"Content-Type": "application/json"}), timeout=300).read())
new = r["result"][BOARD]
gold = json.load(open(f'netlist/{BOARD}_parts_golden.json'))
def sig(p): return (p['dev']['name'], tuple(sorted((q['num'], q['net']) for q in p['pins'])), tuple(sorted(q['num'] for q in p['pins'] if q['nc'])))
G = collections.Counter(sig(p) for p in gold); N = collections.Counter(sig(p) for p in new['parts'])
only_g, only_n = G - N, N - G
nets = {}; cur = None
for ln in new['nl'].splitlines():
    t = ln.strip()
    if t == '(': cur = []
    elif t == ')':
        if cur: nets[cur[0]] = cur[1:]
        cur = None
    elif cur is not None and t: cur.append(t if not cur else t.split()[0])
singles = sorted(n for n, ps in nets.items() if len(ps) == 1)
print(f"{BOARD}: golden {sum(G.values())} parts, new {sum(N.values())} parts")
print("IDENTICAL" if not only_g and not only_n else "MISMATCH")
for s in only_g: print("  missing:", s[0], s[1][:6])
for s in only_n: print("  extra  :", s[0], s[1][:6])
print("single-pin nets:", singles or "none")
open(f'netlist/{BOARD}_relayout.net', 'w', encoding='utf-8').write(new['nl'])
json.dump(new['parts'], open(f'netlist/{BOARD}_parts_relayout.json', 'w'), indent=1)
sys.exit(0 if not only_g and not only_n and not singles else 1)
