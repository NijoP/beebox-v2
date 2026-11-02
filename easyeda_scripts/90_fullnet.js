const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
// compress pin strings to REF-PIN(func)
const out={};
for(const n of nets){ out[n.name]=n.pins.map(p=>{const m=p.match(/^(\S+)\s+\S+-(\S+)\s+\w+$/); return m?(m[1]):p.split(' ')[0];}); }
return out;
