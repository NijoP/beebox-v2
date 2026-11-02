await eda.dmt_EditorControl.openDocument("3d113e2c2213fddc");
await new Promise(r=>setTimeout(r,1500));
const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not on main sheet",DOC};
const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId("cbdd395c0c939208");
const lr=pins.find(p=>p.getState_PinName()==="LR"); const px=lr.getState_X(),py=lr.getState_Y();
const wires=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());
let end=null;const del=[];
for(const w of wires){const ln=w.getState_Line();for(let i=0;i<ln.length;i+=2){if(ln[i]===px&&ln[i+1]===py){del.push(w.getState_PrimitiveId());end=(i===0)?[ln[ln.length-2],ln[ln.length-1]]:[ln[0],ln[1]];}}}
await eda.sch_PrimitiveWire.delete(del);
await eda.sch_PrimitiveWire.create([px,py,end[0],end[1]],"+3V3");
const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],"+3V3");
try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(px===end[0])?90:0,fontSize:5});}catch(e){}
const nl=await eda.sch_Netlist.getNetlist("Protel2");
const ok3=nl.split("(").find(b=>b.includes("U7-2 "));
return {pin:[px,py],end,deleted:del.length,netOfU7_2:ok3?ok3.trim().split(/\r?\n/)[0]:null};
