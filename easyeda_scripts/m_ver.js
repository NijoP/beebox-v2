const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
const nl=await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t);}}
const m={};for(const n of nets)m[n.name]=n.pins.map(p=>p.split(' ')[0]);
const singles=nets.filter(n=>n.pins.length===1).map(n=>n.name).sort();
return {components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length,nets:nets.length,GND:m["GND"].length,P3V3:m["+3V3"].length,U7_on_3V3:m["+3V3"].includes("U7-2"),U7_on_GND:m["GND"].includes("U7-2"),singles,
 chain:["+5V_CHAIN","SENSOR_CLK_C","SENSOR_SYNC_C","SENSOR_DATA_ID","PATCH_DATA_C","PATCH_CLK_C","PATCH_LATCH_C","PATCH_OE_N_C","AUX_ARM"].map(k=>k+":"+m[k].join(","))};
