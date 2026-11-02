const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const singles=nets.filter(n=>n.pins.length===1).map(n=>n.name).sort();
const exp=["IO3","IO45","IO46","SBU1","SBU2"];
return {components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length, totalNets:nets.length, unexpectedSingles:singles.filter(s=>!exp.includes(s)), allSingles:singles};
