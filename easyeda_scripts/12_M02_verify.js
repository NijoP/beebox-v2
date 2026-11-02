const names = await eda.sch_Net.getAllNetsName();
const nets = await eda.sch_Net.getAllNets();
const summary = (nets || []).map(n => ({ net: n.net ?? n.name ?? n.netName, ...n }));
const want = ["+3V3","GND","MCU_EN","MCU_BOOT","SENSOR_DATA_ID","SENSOR_CLK","SENSOR_SYNC",
 "SENSOR_PU_EN","CHAIN_PWR_EN","CHAIN_FAULT_N","SYS_SDA","SYS_SCL","PATCH_SHIFT_DATA",
 "PATCH_SHIFT_CLK","PATCH_LATCH","PATCH_OE_N","DISP_PWR_EN","REG_PS_SYNC","CHG_EN1","CHG_EN2",
 "USB_DM","USB_DP","CHG_STAT_N","CHG_PGOOD_N","REG_PG","GAUGE_ALRT_N","LED_R","LED_G","LED_B",
 "VBAT_SENSE","UART0_TX","UART0_RX"];
const present = want.filter(w => names.includes(w));
const missing = want.filter(w => !names.includes(w));
return { totalNets: names.length, presentCount: present.length, missing, allNames: names.sort() };
