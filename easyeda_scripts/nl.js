const nl = await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/); const nets=[]; let cur=null;
for(const ln of lines){const t=ln.trim();
 if(t==="(")cur={pins:[]}; else if(t===")"){if(cur)nets.push(cur);cur=null;}
 else if(cur){if(!cur.name)cur.name=t; else cur.pins.push(t);}}
const want=["+3V3","GND","VSYS","VBAT","VBUS","+5V0","+5V_CHAIN","USB_DP","USB_DM","CHG_EN1","FB_3V3","FB_5V0","BQ_ISET"];
const map={}; for(const n of nets) map[n.name]=n.pins.length;
return { totalNets: nets.length, key: want.map(w=>w+":"+(map[w]||0)) };
