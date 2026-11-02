const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main",DOC};
async function pinXY(id,key){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const p=ps.find(q=>q.getState_PinNumber()===key||q.getState_PinName()===key);return p?[p.getState_X(),p.getState_Y()]:null;}
async function wiresAt(x,y){const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());const r=[];for(const w of ws){const l=w.getState_Line();if((l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y))r.push(w);}return r;}
async function relabel(id,key,net){const xy=await pinXY(id,key);if(!xy)return "nopin "+key;const ws=await wiresAt(xy[0],xy[1]);if(!ws.length)return "nowire";const l=ws[0].getState_Line();const end=(l[0]===xy[0]&&l[1]===xy[1])?[l[l.length-2],l[l.length-1]]:[l[0],l[1]];
 await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));await eda.sch_PrimitiveWire.create([xy[0],xy[1],end[0],end[1]],net);
 const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(xy[0]===end[0])?90:0,fontSize:5});}catch(e){}return "ok";}
async function removePart(id){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){const ws=await wiresAt(p.getState_X(),p.getState_Y());if(ws.length)await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));}await eda.sch_PrimitiveComponent.delete([id]);}
async function partAt(x,y){const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());const c=cs.find(c=>c.getState_X()===x&&c.getState_Y()===y);return c?c.getState_PrimitiveId():null;}
const r={};
r.fpc=await relabel("3286f368e9df3453","12","GND");
r.pogo=await relabel("e1819e7a52bcb5cd","12","GND");
r.esd=await relabel("7c6ba516b90e8371","D2-","GND");
const r37=await partAt(3050,2290); r.r37=r37; if(r37) await removePart(r37);
return r;
