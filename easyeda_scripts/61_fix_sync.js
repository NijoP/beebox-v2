const fpc="ce815dd3f4fbab2f", pogo="206da01be8fbcb6a";
async function pinXY(id, num){
  const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);
  for(const p of pins){ if(p.getState_PinNumber()===num) return {x:p.getState_X(),y:p.getState_Y(),r:p.getState_Rotation()}; }
  return null;
}
const f5=await pinXY(fpc,"5");
const p5=await pinXY(pogo,"5");
const targets=[f5,p5].filter(Boolean);
// delete wires touching these pins
const wids=await eda.sch_PrimitiveWire.getAllPrimitiveId();
const wires=await eda.sch_PrimitiveWire.get(wids);
const del=[];
for(const w of wires){ let ln=null; try{ln=w.getState_Line();}catch(e){} if(!ln)continue;
  for(let i=0;i<ln.length;i+=2){ for(const t of targets){ if(Math.abs(ln[i]-t.x)<4 && Math.abs(ln[i+1]-t.y)<4){ del.push(w.getState_PrimitiveId()); } } }
}
if(del.length) await eda.sch_PrimitiveWire.delete([...new Set(del)]);
// recreate far-left stubs clear of GND labels
const made=[];
for(const t of targets){
  const ex = t.x - 280;
  await eda.sch_PrimitiveWire.create([t.x, t.y, ex, t.y], "SENSOR_SYNC_C");
  const lb=await eda.sch_PrimitiveAttribute.createNetLabel(ex, t.y, "SENSOR_SYNC_C");
  try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5});}catch(e){}
  made.push([t.x,t.y,"->",ex]);
}
return { f5, p5, wiresDeleted: new Set(del).size, recreated: made };
