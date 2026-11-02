const nl = await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/); const nets=[]; let cur=null;
for(const ln of lines){const t=ln.trim();
 if(t==="(")cur={pins:[]}; else if(t===")"){if(cur)nets.push(cur);cur=null;}
 else if(cur){if(!cur.name)cur.name=t; else cur.pins.push(t);}}
const hit=nets.filter(n=>/SYNC|SENSOR/i.test(n.name));
return hit.map(n=>({net:n.name, pins:n.pins}));
