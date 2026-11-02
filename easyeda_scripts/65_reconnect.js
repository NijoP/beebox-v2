const pts=[[2170,1700],[2330,1365],[2320,855],[1910,1460]];
for(const [x,y] of pts){
  await eda.sch_PrimitiveWire.create([x,y,x-160,y],"SENSOR_SYNC_C");
  const lb=await eda.sch_PrimitiveAttribute.createNetLabel(x-160,y,"SENSOR_SYNC_C");
  try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5});}catch(e){}
}
const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const m={};for(const n of nets)m[n.name]=n.pins;
return { SYNC_C:(m["SENSOR_SYNC_C"]||[]).length, pins:(m["SENSOR_SYNC_C"]||[]), GND:(m["GND"]||[]).length };
