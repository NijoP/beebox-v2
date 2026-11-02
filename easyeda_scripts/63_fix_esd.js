const tx=1910, ty=1460; // ESD1 D1- = SENSOR_SYNC_C
// delete wires touching the D1- pin
const wids=await eda.sch_PrimitiveWire.getAllPrimitiveId();
const wires=await eda.sch_PrimitiveWire.get(wids);
const delW=[];
for(const w of wires){ let ln=null; try{ln=w.getState_Line();}catch(e){} if(!ln)continue;
  for(let i=0;i<ln.length;i+=2){ if(Math.abs(ln[i]-tx)<4&&Math.abs(ln[i+1]-ty)<4) delW.push(w.getState_PrimitiveId()); } }
if(delW.length) await eda.sch_PrimitiveWire.delete([...new Set(delW)]);
// delete stray net labels at old stub end (1810,1460) only (not the GND label at 1450)
const aids=await eda.sch_PrimitiveAttribute.getAllPrimitiveId();
const attrs=await eda.sch_PrimitiveAttribute.get(aids);
const delA=[];
for(const a of attrs){ let x,y; try{x=a.getState_X();y=a.getState_Y();}catch(e){continue;}
  if(Math.abs(x-1810)<6 && Math.abs(y-1460)<4) delA.push(a.getState_PrimitiveId()); }
for(const id of delA){ try{await eda.sch_PrimitiveAttribute.delete([id]);}catch(e){} }
// recreate clear stub
await eda.sch_PrimitiveWire.create([tx,ty,1630,ty],"SENSOR_SYNC_C");
const lb=await eda.sch_PrimitiveAttribute.createNetLabel(1630,ty,"SENSOR_SYNC_C");
try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5});}catch(e){}
return { wiresDeleted:new Set(delW).size, labelsDeleted:delA.length };
