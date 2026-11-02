const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const m={};for(const n of nets)m[n.name]=n.pins.map(p=>{const s=p.split(' ');return s[0]+"("+s[1].split('-').pop()+")";});
const o={components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length,nets:nets.length,GND:(m.GND||[]).length,singles:nets.filter(n=>n.pins.length===1).map(n=>n.name).sort()};
for(const k of ["BAT_IN","CHG_IN","BAT_F","VIN_PROT","SW_5V","CB_5V","FB_5V","+5V0","VBUS","VBUS_DET","VBAT_SENSE","+3V3","+5V_CHAIN","PWR_EN","BTN","VSYS","VBAT","CHG_STAT_N"])o[k]=m[k]||"(absent)";
return o;
