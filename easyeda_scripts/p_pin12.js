await eda.dmt_EditorControl.openDocument("df98848100b24fd4"); await new Promise(r=>setTimeout(r,1500)); const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"not patch",DOC};
async function pinXY(id,key){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const p=ps.find(q=>q.getState_PinNumber()===key||q.getState_PinName()===key);return p?[p.getState_X(),p.getState_Y()]:null;}
async function wiresAt(x,y){const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());const r=[];for(const w of ws){const l=w.getState_Line();if((l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y))r.push(w);}return r;}
async function relabel(id,key,net){const xy=await pinXY(id,key);if(!xy)return "nopin "+key;const ws=await wiresAt(xy[0],xy[1]);if(!ws.length)return "nowire";const l=ws[0].getState_Line();const end=(l[0]===xy[0]&&l[1]===xy[1])?[l[l.length-2],l[l.length-1]]:[l[0],l[1]];
 await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));await eda.sch_PrimitiveWire.create([xy[0],xy[1],end[0],end[1]],net);
 const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(xy[0]===end[0])?90:0,fontSize:5});}catch(e){}return "ok";}
async function removePart(id){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){const ws=await wiresAt(p.getState_X(),p.getState_Y());if(ws.length)await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));}await eda.sch_PrimitiveComponent.delete([id]);}
async function partAt(x,y){const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());const c=cs.find(c=>c.getState_X()===x&&c.getState_Y()===y);return c?c.getState_PrimitiveId():null;}
const r={jin:await relabel("2a6508df78168858","12","GND"), jout:await relabel("a114037565df532e","12","GND")};
async function T(x,y,s,sz){await eda.sch_PrimitiveText.create(x,y,s,0,"#1569C7",null,sz||14,true,false,false);}
await T(150,1640,"CHAIN IN (from main / previous pad)  |  CHAIN OUT (to next pad)");
await T(800,1640,"LOCAL POWER  +5V_CHAIN -> TPS62840 -> +3V3_LOCAL");
await T(750,1130,"SHIFT REGISTER 74LV595  (pin8 cascade)");
await T(1450,1640,"PRIMARY SENSOR + ID");
await T(150,880,"FAIL-SAFE PULL-DOWNS / OE PULL-UP");
await T(1400,880,"PRIMARY PASSIVES");
await T(450,560,"SENSOR MODULE HEADER  J1: 1 3V3 | 2 GND | 3 SDA | 4 SCL  (280 mA limit)");
await T(1400,1180,"VARIANT T/RH: fit SHT45 + R 22k, DNI mic + R 68k",8);
await T(1400,1155,"VARIANT MIC : fit ICS-43434 + R 68k, DNI SHT45 + R 22k",8);
await T(150,120,"Q0 PRI_PWR | Q1 PRI_SIG | Q2 ID | Q3 MOD_PWR | Q4 MOD_SIG | Q5-Q7 spare",8);
return r;
