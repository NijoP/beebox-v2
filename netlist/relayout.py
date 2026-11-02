"""Subsystem relayout of a BeeBox schematic sheet from its golden snapshot.
Usage: python netlist/relayout.py main|patch
- clears the sheet (keeps the drawing frame), rebuilds every part with identical pin->net mapping
- one framed + titled block per subsystem, ICs in a row, passives in a grid under them
- re-applies No-Connect flags; then the caller verifies connectivity vs golden."""
import json, sys, urllib.request

BOARD = sys.argv[1]
DOC = {"main": "3d113e2c2213fddc", "patch": "df98848100b24fd4", "splitter": "86264b6bbddc15ff", "c_patch": "dc18354c68ca78d6", "c_split": "68d2d656cee68532"}[BOARD]
LIB = "0819f05c4eef4c71ace90d822a990e87"
UUID = {
 "ESP32-S3-WROOM-1-N16R2": "66b3838809bd4c87b584f0ab36e52b0a", "CL21A226MAQNNNE": "9583d53cf848486a9d523fcaf153a729",
 "CL05B104KO5NNNC": "2eaf9ba597d94b93917d2b625402739a", "0402WGF1002TCE": "ffb8c4ed7d7244bc918e37313d4ca373",
 "CL05A105KA5NQNC": "115c3169541a48b3ba9c88523dfbcdee", "CL05B103KB5NNNC": "5da69c8ac73640e59896b82d2fd17bab",
 "TS-1187A-B-A-B": "38630614a58f4650bd5eefe9a5f79d9a", "USB4105-GF-A": "ea8fe62438f94521b482dce770be0817",
 "TPS63802DLAR": "009be0afb86044d8a62900f07f64ed07", "TPS2553DBVR-1": "921d0fc4bb4347dca0bd09d3e42f87c6",
 "TPD2E2U06DRLR": "ef0c95266254481fa44a3c27c326d03c", "0402WGF1003TCE": "a8fe6fdf29924f6c9dcbfa5a84a02fec",
 "0402WGF5101TCE": "3614094eae9e47dbbbde1f877b2c30ef", "0402WGF5623TCE": "311efdff25ca4ed086c602bd49bb7bb2",
 "0402WGF7682TCE": "8866aaee3e9549c88c7f31286c8f6648", "0402WGF1004TCE": "72d71792aca642bfa6283f6c1d1961c3",
 "CL21A106KOQNNNE": "b170840a71594691a5c5caad0ed65e15", "SWPA4030S1R5NT": "64e29f11414b4167aed61e2847fee31d",
 "0402WGF1001TCE": "17aa4e57569d4d22adc28d945ba5559d", "KT-0603G": "e8b07cfe9d254e27bcc3c1116be11e2b",
 "KT-0603R": "015cbd77079d4b0ea5c81b79fefdcea3", "KT-0603B": "1a0921f6f61a426b8b7f0aa3c5392e45",
 "0402WGF330JTCE": "1f1e772b30d54f119255cd0da7dd806e", "0402WGF2201TCE": "ab7b2f8b72664e3fb9f5bed85b07623d",
 "FH34SRJ-12S-0.5SH(50)": "56d0813866664aa693617cc16753ea47", "HDR-M_2.54_2x6": "677932d4aced4f289d0065197a57e940",
 "TPD4E05U06DQAR": "71bd5434893a460c8be823067c2f4e47", "SHT45-AD1B-R2": "d2ae2b1001894b88873ae8d8ab95ec2d",
 "ICS-43434_C5656610": "ea6d3dad996d464d91983d780e0fb112", "HDR-M_2.54_1x4P_C9900236824": "48c3a4f6cdc941c7b2c8ede78c08955e",
 "BAT54WS": "f9c1784dc49145d1a3ff819c8a8f0622", "AO3400A": "abeeb35504de41a282e8a5f4b5f7cca9",
 "B2B-XH-A(LF)(SN)": "4d018698282b47d4893c87aca1c32f67", "SMD1812P110TF/33": "6800a564dc6b4d7a98c275222893ecd6",
 "SS34": "b8a5c6cf81fb4f7e8e47029a0abe40d8", "SMBJ26A": "fadafc8c52304b4fbd6862ec903a02ce",
 "LMR51420XDDCR": "9f7005c05c264d3aa9ef2db611895932", "CL31A106KBHNNNE": "7d63f541153841429f3d9086c7babcc4",
 "CL05B104KB54PNC": "4724975b6c2a4e8dbf5d2a441323882f", "SWPA6045S100MT": "438ae48248e246a9a68a6c56e8036c58",
 "0402WGF1372TCE": "a7dea258d14247b0968153eb1b4aa039", "SS14_C2480": "93c829a348834d679d8d8a059ea91636",
 "0402WGF4702TCE": "67bcedcf1a474734ac65b62dc52125d6", "DC-005-5A-2.0": "9944dd3d7f214ad59de4760c03e84c88",
 "BSMD1812-200-33V": "6c49915692fa40ac8af2f201facd8b0a", "RV-3028-C7": "4c439d145251422d86fe12a14a648115",
 "CR2032-BS-6-1": "6c6537e92f8045d7b2cdb8b0ea44de27", "TPS62840DLCR": "b902ba68e4fd4b048ff83be09f873cbc",
 "SN74LV595APWR": "f7296b70af1e425f9d78faa598566f18", "TPS22918DBVR": "e30f49de94c341bea49e53f3016845f9",
 "TMUX1511PWR": "4ab6cbe4c9f94cc58a3871090025db9c", "B4B-PH-K-S(LF)(SN)": "f61f530515f040b2b0bc6d26df5b0e4f",
 "0402WGF2673TCE": "87785ec059df4357ad5cfe4df6f7198d", "SWPA3015S2R2NT": "8c67669561cd4d729d3c04e95ece45c1",
 "CL10A475KO8NNNC": "15e034b4cdd64e4292f01bb80f418a78", "0402B102K500NT": "0534450544ca4f1fbbfa4b197d2a3e2d",
 "0402WGF2202TCE": "2b4222cc312549c69f08ca8d58423a8a", "0402WGF9312TCE": "59ffbf90b9ad49aaa3a5e1cedea45c77",
 "SN74LVC1G08DBVR": "aa7194888c6b459094e5437dc75d23f0", "0402WGF6802TCE": "417e43b7b2f04d94a9c8e8175d11e298"}

BLOCKS = {
 "c_patch": [('CHAIN IN / OUT', ['FPC101', 'FPC102']), ('LOCAL POWER 3V3', ['U101', 'L101', 'R101', 'C101', 'C102', 'C103', 'C104']), ('SHIFT REGISTER', ['U102', 'R102', 'R103', 'C105', 'R104', 'R105', 'R106', 'R107', 'R108', 'R110', 'R113']), ('ID & PRESENCE', ['R109', 'R114', 'R115', 'R119', 'R120']), ('MIC POWER', ['U103', 'C106', 'C107', 'C108']), ('MIC SWITCH + MICROPHONE', ['U110', 'U104', 'U106', 'C115', 'C109', 'C111', 'R116', 'R117', 'R118']), ('SENSOR / AUX POWER', ['U107', 'R111', 'R112', 'C112', 'C113']), ('I2C SWITCH + SHT45 + AUX HEADER', ['U111', 'U108', 'U105', 'U109', 'D101', 'C116', 'C114', 'C110'])],
 "c_split": [('FPC CONNECTOR (CHAIN SIDE)', ['J201']), ('POGO CONNECTOR (PATCH SIDE)', ['J202']), ('ESD PROTECTION', ['D201', 'D202'])],
 "main": [
  ("BATTERY INPUT & PROTECTION", ["CN1", "DC1", "U4", "F1", "D6", "D7", "C7", "C8", "C11", "R20", "R7", "C15"]),
  ("5V BUCK REGULATOR", ["U2", "L2", "C18", "R3", "R4", "C19", "C20"]),
  ("3V3 REGULATOR", ["U3", "L1", "R15", "R16", "C9", "C10", "R11", "R43"]),
  ("POWER BUTTON", ["SW3", "Q1", "D4", "D5", "R22", "R23", "R24"]),
  ("USB-C", ["USB1", "D1", "D8", "R13", "R14", "C6", "R5", "R6"]),
  ("MCU ESP32-S3", ["U1", "C1", "C2", "C3", "R1", "C4", "SW1", "R2", "C5", "SW2"]),
  ("CHAIN POWER SWITCH", ["U5", "R19", "R12", "R42", "C12", "C13", "C14"]),
  ("CHAIN SIGNAL CONDITIONING", ["R30", "R31", "R32", "R33", "R34", "R35", "R36", "R38", "R39"]),
  ("CHAIN CONNECTORS FPC / POGO + ESD", ["FPC1", "H1", "D2", "D3"]),
  ("ONBOARD SENSORS", ["U6", "C16", "U7", "C17"]),
  ("DISPLAY / SYSTEM I2C", ["U8", "R40", "R41"]),
  ("RTC", ["U9", "B1", "C21", "C22", "R8"]),
  ("STATUS LEDS", ["LED1", "LED3", "LED4", "LED5", "R25", "R27", "R28", "R29"])],
 "splitter": [
  ("FPC CONNECTOR (CHAIN SIDE)", ["J1"]),
  ("POGO CONNECTOR (PATCH SIDE)", ["J2"]),
  ("ESD PROTECTION", ["D1", "D2"])],
 "patch": [
  ("CHAIN IN / OUT", ["FPC1", "FPC2"]),
  ("LOCAL POWER 3V3", ["U1", "L1", "R1", "C1", "C2", "C3", "C4"]),
  ("SHIFT REGISTER", ["U2", "R2", "R3", "C5", "R4", "R5", "R6", "R7", "R8", "R10", "R13"]),
  ("ID & PRESENCE", ["R9", "R14", "R15", "R19", "R20"]),
  ("MIC POWER", ["U3", "C6", "C7", "C8"]),
  ("MIC SWITCH + MICROPHONE", ["U10", "U4", "U6", "C15", "C9", "C11", "R16", "R17", "R18"]),
  ("SENSOR / AUX POWER", ["U7", "R11", "R12", "C12", "C13"]),
  ("I2C SWITCH + SHT45 + AUX HEADER", ["U11", "U8", "U5", "U9", "D1", "C16", "C14", "C10"])]}

ROW_MAX = {"main": 5200, "patch": 3600, "splitter": 3000, "c_patch": 3600, "c_split": 3000}[BOARD]
TOP = {"main": 3250, "patch": 2350, "splitter": 1500, "c_patch": 2350, "c_split": 1500}[BOARD]

def ex(code, timeout=280):
    req = urllib.request.Request("http://127.0.0.1:49620/execute", data=json.dumps({"code": code}).encode(),
                                 headers={"Content-Type": "application/json"})
    r = json.loads(urllib.request.urlopen(req, timeout=timeout).read())
    if not r.get("success"): raise SystemExit("EDA error: " + str(r.get("error")))
    return r["result"]

GUARD_OPEN = f'await eda.dmt_EditorControl.openDocument("{DOC}"); await new Promise(r=>setTimeout(r,1200)); const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="{DOC}") return {{error:"wrong sheet"}};\n'
GUARD = f'const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="{DOC}") return {{error:"wrong sheet"}};\n'

BLOCK_JS = r'''
const LIB="__LIB__"; const SPEC=__SPEC__; const X0=__X0__, YT=__YT__;
const CH=5.5, ST=[110,170], PAD=60, TITLE=90, GAPX=120, PITCH=400, ROWH=125;
function isIC(p){return p.pins.length>4 || /^(U|FPC|H|USB|CN|DC|Q|SW|B)/.test(p.ref) && p.pins.length>2;}
async function labelPins(id,map){const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const g={};for(const p of pins){const r=""+p.getState_Rotation();(g[r]=g[r]||[]).push(p);}
 for(const r of Object.keys(g)){const arr=g[r];if(r==="180"||r==="0")arr.sort((a,b)=>b.getState_Y()-a.getState_Y());else arr.sort((a,b)=>a.getState_X()-b.getState_X());
  for(let k=0;k<arr.length;k++){const p=arr[k];const net=map[p.getState_PinNumber()];if(net==null)continue;const x=p.getState_X(),y=p.getState_Y(),R=+r,s=ST[k%2];let ex=x,ey=y,lr=0;
   if(R===180)ex=x-s;else if(R===0)ex=x+s;else if(R===270){ey=y-(s-30);lr=90;}else{ey=y+(s-30);lr=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:lr,fontSize:5});}catch(e){}}}}
async function ext(id,map){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);let L=1e9,R=-1e9,T=-1e9,B=1e9;
 for(const p of ps){const x=p.getState_X(),y=p.getState_Y(),r=p.getState_Rotation(),n=(map[p.getState_PinNumber()]||"").length*CH;
  L=Math.min(L,x);R=Math.max(R,x);T=Math.max(T,y);B=Math.min(B,y);
  if(r===180)L=Math.min(L,x-ST[1]-10);if(r===0)R=Math.max(R,x+ST[1]+n);if(r===270)B=Math.min(B,y-ST[1]-n);if(r===90)T=Math.max(T,y+ST[1]+n);}
 return {L:L-40,R:R+40,T:T+40,B:B-40};}
async function nc(id,list){if(!list.length)return;const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){if(list.includes(p.getState_PinNumber())){p.setState_NoConnected(true);await p.done();}}}
const ics=SPEC.parts.filter(isIC), pas=SPEC.parts.filter(p=>!isIC(p));
let cx=X0+PAD, rowB=YT-TITLE, maxR=X0+PAD; const REF={};
for(const p of ics){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:p.uuid},X0+3000,YT-3000,"",0,false,true,true);const id=c.getState_PrimitiveId();
 const e=await ext(id,p.map);const nx=X0+3000+(cx-e.L), ny=YT-3000+((YT-TITLE)-e.T);
 await eda.sch_PrimitiveComponent.modify(id,{x:nx,y:ny,designator:p.ref});const e2=await ext(id,p.map);
 await labelPins(id,p.map);await nc(id,p.nc);REF[p.ref]=(await eda.sch_PrimitiveComponent.get(id)).getState_Designator();cx=e2.R+GAPX;maxR=Math.max(maxR,e2.R);rowB=Math.min(rowB,e2.B);}
const icW=Math.max(maxR-X0, 2*PITCH+PAD);
const cols=Math.max(2,Math.min(5,Math.floor((icW-PAD)/PITCH)));
let y=(ics.length?rowB:YT-TITLE)-90, i=0;
for(const p of pas){const col=i%cols,row=Math.floor(i/cols);const x=X0+PAD+170+col*PITCH, yy=y-row*ROWH;
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:p.uuid},x,yy,"",0,false,true,true);await eda.sch_PrimitiveComponent.modify(c.getState_PrimitiveId(),{designator:p.ref});await labelPins(c.getState_PrimitiveId(),p.map);await nc(c.getState_PrimitiveId(),p.nc);REF[p.ref]=(await eda.sch_PrimitiveComponent.get(c.getState_PrimitiveId())).getState_Designator();i++;}
const rows=Math.ceil(pas.length/cols); const bottom=pas.length? y-(rows-1)*ROWH-80 : rowB-20;
const W=Math.max(icW+PAD, cols*PITCH+PAD+40), H=YT-bottom;
await eda.sch_PrimitiveRectangle.create(X0,YT,W,H,0,0,"#2E5E8C",null,1,null,null);
await eda.sch_PrimitiveText.create(X0+20,YT-45,SPEC.title,0,"#2E5E8C",null,12,true,false,false);
return {W,H,REF};
'''

def main():
    parts = {p["ref"]: p for p in json.load(open(f"netlist/{BOARD}_parts_golden.json"))}
    blocks = BLOCKS[BOARD]
    listed = [r for _, refs in blocks for r in refs]
    missing = sorted(set(parts) - set(listed)); extra = sorted(set(listed) - set(parts))
    dup = sorted({r for r in listed if listed.count(r) > 1})
    if missing or extra or dup: raise SystemExit(f"block map error missing={missing} extra={extra} dup={dup}")
    # clear sheet (keep drawing frame = components without designator)
    print(ex(GUARD_OPEN + r'''
const w=await eda.sch_PrimitiveWire.getAllPrimitiveId(); if(w.length) await eda.sch_PrimitiveWire.delete(w);
const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const del=cs.filter(c=>c.getState_Designator()).map(c=>c.getState_PrimitiveId()); if(del.length) await eda.sch_PrimitiveComponent.delete(del);
const t=await eda.sch_PrimitiveText.getAllPrimitiveId(); if(t.length) await eda.sch_PrimitiveText.delete(t);
const rc=await eda.sch_PrimitiveRectangle.getAllPrimitiveId(); if(rc.length) await eda.sch_PrimitiveRectangle.delete(rc);
return {wires:w.length,parts:del.length,texts:t.length,rects:rc.length,left:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};'''))
    x, y, rowH = 100, TOP, 0
    refmap = {}
    for title, refs in blocks:
        spec = {"title": title, "parts": [{"ref": r, "uuid": UUID[parts[r]["dev"]["name"]],
                 "map": {q["num"]: q["net"] for q in parts[r]["pins"] if q["net"]},
                 "nc": [q["num"] for q in parts[r]["pins"] if q["nc"]], "pins": parts[r]["pins"]} for r in refs]}
        js = BLOCK_JS.replace("__LIB__", LIB).replace("__SPEC__", json.dumps(spec)).replace("__X0__", str(x)).replace("__YT__", str(y))
        nrect_before = ex(GUARD + 'return (await eda.sch_PrimitiveRectangle.getAllPrimitiveId()).length;')
        try:
            res = ex(GUARD + js)
        except Exception as e:
            import time
            for _ in range(12):
                time.sleep(10)
                info = ex(GUARD + 'const ids=await eda.sch_PrimitiveRectangle.getAllPrimitiveId(); const r=ids.length?await eda.sch_PrimitiveRectangle.get(ids[ids.length-1]):null; return {n:ids.length,w:r?r.getState_Width():0,h:r?r.getState_Height():0};')
                if info['n'] == nrect_before + 1: break
            else:
                raise SystemExit(f'block {title} did not complete: {e}')
            res = {'W': info['w'], 'H': info['h'], 'REF': {r: r for r in refs}}
            print('   (bridge timeout; block confirmed complete via frame)')
        refmap.update(res['REF'])
        print(f"{title:38s} at ({x},{y}) size {res['W']:.0f}x{res['H']:.0f}")
        x += res["W"] + 150; rowH = max(rowH, res["H"])
        if x > ROW_MAX: x, y, rowH = 100, y - rowH - 150, 0
    json.dump(refmap, open(f'netlist/{BOARD}_refmap.json','w'), indent=1)
    print('refmap saved', len(refmap))

if __name__ == "__main__":
    main()
