const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
// pogo = the 2x6 header placed at connectors (ref likely H2 or similar). find header pins with HDR-M_2.54_2x6 that also share chain nets
const out={};
for(const n of nets){for(const p of n.pins){const m=p.match(/^(H\d+)-(\d+)\s+HDR-M_2\.54_2x6\b/); if(m){ out[m[1]]=out[m[1]]||{}; out[m[1]][m[2]]=n.name; }}}
return out;
