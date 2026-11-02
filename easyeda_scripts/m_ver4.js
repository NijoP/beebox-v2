const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const m={};for(const n of nets)m[n.name]=n.pins.map(p=>{const s=p.split(' ');return s[0]+"("+s[1].split('-').pop()+")";});
const gfun=["GND","VSS","EP","AGND"];const shorts=[];
for(const n of nets){if(n.name==="GND")continue;for(const p of n.pins){const f=p.split(' ')[1].split('-').pop();if(gfun.includes(f))shorts.push(n.name+":"+p.split(' ')[0]);}}
const o={components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length,nets:nets.length,GND:m.GND.length,shorts,singles:nets.filter(n=>n.pins.length<2).map(n=>n.name).sort()};
for(const k of ["VBAT_RTC","RTC_INT_N","SYS_SDA","SYS_SCL","+3V3"])o[k]=m[k];
o.rtcPinsOnGND=m.GND.filter(p=>p.startsWith("U")&&/(VSS|EVI)/.test(p));
return o;
