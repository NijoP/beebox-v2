await eda.sch_PrimitiveWire.create([920,3250,1090,3250],"GND");
const L=await eda.sch_PrimitiveAttribute.createNetLabel(1090,3250,"GND"); try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{fontSize:5});}catch(e){}
const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t.split(' ')[0]);}}
const m={};for(const n of nets)m[n.name]=n.pins;
return {GND:m.GND.length,dc3onGND:m.GND.includes("DC1-3"),CHG_IN:m.CHG_IN,BAT_IN:m.BAT_IN,nets:nets.length,singles:nets.filter(n=>n.pins.length<2).map(n=>n.name)};
