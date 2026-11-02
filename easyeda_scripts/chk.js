const nl = await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/); const nets=[]; let cur=null;
for(const ln of lines){const t=ln.trim();
 if(t==="(")cur={pins:[]}; else if(t===")"){if(cur)nets.push(cur);cur=null;}
 else if(cur){if(!cur.name)cur.name=t; else cur.pins.push(t);}}
const m={}; for(const n of nets)m[n.name]=n.pins;
const sc=m["SENSOR_SYNC_C"]||[];
return { SENSOR_SYNC_C_count: sc.length, SENSOR_SYNC_C_pins: sc, GND_count:(m["GND"]||[]).length };
