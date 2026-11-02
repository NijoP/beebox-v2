const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const m={};for(const n of nets)m[n.name]=n.pins.length;
return {CHAIN_PWR_EN:m["CHAIN_PWR_EN"],REG_PS_SYNC:m["REG_PS_SYNC"],components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};
