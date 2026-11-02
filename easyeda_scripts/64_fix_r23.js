const tx=2170, ty=1700; // R23 right pin = SENSOR_SYNC_C
const wids=await eda.sch_PrimitiveWire.getAllPrimitiveId();
const wires=await eda.sch_PrimitiveWire.get(wids);
const delW=[];
for(const w of wires){ let ln=null; try{ln=w.getState_Line();}catch(e){} if(!ln)continue;
  for(let i=0;i<ln.length;i+=2){ if(Math.abs(ln[i]-tx)<6&&Math.abs(ln[i+1]-ty)<6) delW.push(w.getState_PrimitiveId()); } }
if(delW.length) await eda.sch_PrimitiveWire.delete([...new Set(delW)]);
await eda.sch_PrimitiveWire.create([tx,ty,tx+200,ty],"SENSOR_SYNC_C");
const lb=await eda.sch_PrimitiveAttribute.createNetLabel(tx+200,ty,"SENSOR_SYNC_C");
try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5});}catch(e){}
return { wiresDeleted:new Set(delW).size };
