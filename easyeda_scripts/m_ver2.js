const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const m={};for(const n of nets)m[n.name]=n.pins.map(p=>{const s=p.split(' ');return s[0]+"("+s[1].split('-').pop()+")";});
const show=["VBUS","VBAT","VSYS","PWR_EN","BTN","PWR_HOLD","PWR_BTN","+5V0","+5V_CHAIN","CHAIN_PWR_EN","CHAIN_FAULT_N","AUX_ARM"];
const o={components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length,nets:nets.length,GND:(m.GND||[]).length,singles:nets.filter(n=>n.pins.length===1).map(n=>n.name)};
for(const k of show)o[k]=m[k]||"(absent)";
o.FPC12_onGND=(m.GND||[]).some(p=>p.startsWith("FPC1-12"));o.H1_12_onGND=(m.GND||[]).some(p=>p.startsWith("H1-12"));
return o;
