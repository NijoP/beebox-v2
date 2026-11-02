await eda.dmt_EditorControl.openDocument("df98848100b24fd4"); await new Promise(r=>setTimeout(r,1500)); const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"not patch",DOC};
async function pinXY(id,key){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const p=ps.find(q=>q.getState_PinNumber()===key||q.getState_PinName()===key);return p?[p.getState_X(),p.getState_Y()]:null;}
async function wiresAt(x,y){const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());const r=[];for(const w of ws){const l=w.getState_Line();if((l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y))r.push(w);}return r;}
async function relabel(id,key,net){const xy=await pinXY(id,key);if(!xy)return "nopin "+key;const ws=await wiresAt(xy[0],xy[1]);if(!ws.length)return "nowire";const l=ws[0].getState_Line();const end=(l[0]===xy[0]&&l[1]===xy[1])?[l[l.length-2],l[l.length-1]]:[l[0],l[1]];
 await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));await eda.sch_PrimitiveWire.create([xy[0],xy[1],end[0],end[1]],net);
 const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(xy[0]===end[0])?90:0,fontSize:5});}catch(e){}return "ok";}
async function removePart(id){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){const ws=await wiresAt(p.getState_X(),p.getState_Y());if(ws.length)await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));}await eda.sch_PrimitiveComponent.delete([id]);}
async function partAt(x,y){const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());const c=cs.find(c=>c.getState_X()===x&&c.getState_Y()===y);return c?c.getState_PrimitiveId():null;}
const r={};
const sht="d94ac63391587a69";
r.sda=await relabel(sht,"SDA","MOD_SDA"); r.scl=await relabel(sht,"SCL","MOD_SCL"); r.vdd=await relabel(sht,"VDD","MOD_3V3");
const csht=await partAt(1450,700); r.csht=csht; if(csht) r.cshtl=await relabel(csht,"1","MOD_3V3");
const r68=await partAt(2200,700); r.r68=r68; if(r68) await removePart(r68);
const ts=await eda.sch_PrimitiveText.get(await eda.sch_PrimitiveText.getAllPrimitiveId());
for(const t of ts){const c=t.getState_Content();if(c.startsWith("VARIANT")) await eda.sch_PrimitiveText.delete([t.getState_PrimitiveId()]);}
await eda.sch_PrimitiveText.create(1400,1180,"Every patch: SHT45 (local I2C, 0x44 reserved) + ICS-43434 (I2S) both fitted; ID = 22k",0,"#1569C7",null,8,true,false,false);
return r;
