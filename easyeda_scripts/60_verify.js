const nl = await eda.sch_Netlist.getNetlist("Protel2");
const lines=nl.split(/\r?\n/); const nets=[]; let cur=null;
for(const ln of lines){const t=ln.trim();
 if(t==="(")cur={pins:[]}; else if(t===")"){if(cur)nets.push(cur);cur=null;}
 else if(cur){if(!cur.name)cur.name=t; else cur.pins.push(t);}}
const map={}; for(const n of nets) map[n.name]=n.pins.length;
const singles = nets.filter(n=>n.pins.length===1).map(n=>n.name).sort();
const expectedSpares = ["IO3","IO45","IO46","IO47","IO48","SBU1","SBU2"];
const unexpectedSingles = singles.filter(s=>!expectedSpares.includes(s));
const key=["+3V3","GND","VSYS","VBUS","VBAT","+5V0","+5V_CHAIN","FB_3V3","FB_5V0",
 "SENSOR_CLK_C","SENSOR_SYNC_C","SENSOR_DATA_ID","PATCH_DATA_C","PATCH_CLK_C","PATCH_LATCH_C","PATCH_OE_N_C","AUX_ARM",
 "SYS_SDA","SYS_SCL","SENSOR_CLK","SENSOR_SYNC","SENSOR_PU_EN","L_3V3A","L_3V3B","L_5V0",
 "LEDR_A","LEDG_A","LEDB_A","CHAIN_PWR_EN","CHAIN_FAULT_N","REG_PG","REG_PS_SYNC"];
return {
  totalNets: nets.length,
  components: (await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length,
  unexpectedSingles,
  allSingles: singles,
  keyCounts: key.map(k=>k+":"+(map[k]||0))
};
