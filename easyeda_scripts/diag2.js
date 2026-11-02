const nl = await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/); const nets=[]; let cur=null;
for(const ln of lines){const t=ln.trim();
 if(t==="(")cur={pins:[]}; else if(t===")"){if(cur)nets.push(cur);cur=null;}
 else if(cur){if(!cur.name)cur.name=t; else cur.pins.push(t);}}
function find(tok){return nets.filter(n=>n.pins.some(p=>p.includes(tok))).map(n=>({net:n.name,pins:n.pins}));}
return { R23: find("R23-"), FPC5: find("FH34SRJ-12S-0.5SH(50)-5"), POGO5: find("2x6-5"), ESD1_D1m: nets.filter(n=>n.pins.some(p=>/TPD4E05U06DQAR-D1-/.test(p))).map(n=>({net:n.name,pins:n.pins})) };
