const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
async function pinXY(id,key){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const p=ps.find(q=>q.getState_PinNumber()===key||q.getState_PinName()===key);return p?[p.getState_X(),p.getState_Y()]:null;}
async function wiresAt(x,y){const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());const r=[];for(const w of ws){const l=w.getState_Line();if((l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y))r.push(w);}return r;}
async function relabel(id,key,net){const xy=await pinXY(id,key);if(!xy)return "nopin "+key;const ws=await wiresAt(xy[0],xy[1]);if(!ws.length)return "nowire";const l=ws[0].getState_Line();const end=(l[0]===xy[0]&&l[1]===xy[1])?[l[l.length-2],l[l.length-1]]:[l[0],l[1]];
 await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));await eda.sch_PrimitiveWire.create([xy[0],xy[1],end[0],end[1]],net);
 const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(xy[0]===end[0])?90:0,fontSize:5});}catch(e){}return "ok";}
async function removePart(id){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){const ws=await wiresAt(p.getState_X(),p.getState_Y());if(ws.length)await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));}await eda.sch_PrimitiveComponent.delete([id]);}
async function partAt(x,y){const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());const c=cs.find(c=>c.getState_X()===x&&c.getState_Y()===y);return c?c.getState_PrimitiveId():null;}
const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const byD=d=>{const c=cs.find(c=>c.getState_Designator()===d);return c?c.getState_PrimitiveId():null;};
const sw3=byD("SW3"),u1=byD("U1");
const r={};
r.u3=await relabel("4c7cbec66e4c5e32","VIN","+5V0");
r.c9=await relabel("783672942319df6d","1","+5V0");
r.swA=await relabel(sw3,"A","+5V0"); r.swC=await relabel(sw3,"C","+5V0");
r.r20=await relabel("384bf5420f2ad8b2","1","VIN_PROT");
r.io21=await relabel(u1,"IO21","VBUS_DET"); r.io17=await relabel(u1,"IO17","IO17"); r.io18=await relabel(u1,"IO18","IO18"); r.io35=await relabel(u1,"IO35","IO35");
return r;
