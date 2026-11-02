const compId = "9cbf883ff1490a85";
const NET = {
  "GND": "GND", "3V3": "+3V3", "EN": "MCU_EN", "IO0": "MCU_BOOT",
  "IO1": "SENSOR_DATA_ID", "IO2": "VBAT_SENSE", "IO4": "SENSOR_CLK", "IO5": "SENSOR_SYNC",
  "IO6": "SENSOR_PU_EN", "IO7": "CHAIN_PWR_EN", "IO8": "CHAIN_FAULT_N", "IO9": "SYS_SDA",
  "IO10": "SYS_SCL", "IO11": "PATCH_SHIFT_DATA", "IO12": "PATCH_SHIFT_CLK", "IO13": "PATCH_LATCH",
  "IO14": "PATCH_OE_N", "IO15": "DISP_PWR_EN", "IO16": "REG_PS_SYNC", "IO17": "CHG_EN1",
  "IO18": "CHG_EN2", "IO19": "USB_DM", "IO20": "USB_DP", "IO21": "CHG_STAT_N",
  "IO35": "CHG_PGOOD_N", "IO36": "REG_PG", "IO37": "GAUGE_ALRT_N", "IO39": "LED_R",
  "IO40": "LED_G", "IO41": "LED_B", "TXD0": "UART0_TX", "RXD0": "UART0_RX"
};
const pins = await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(compId);
const info = pins.map(p => ({ p, name: p.getState_PinName(), x: p.getState_X(), y: p.getState_Y(), rot: p.getState_Rotation() }));
const bottoms = info.filter(i => i.rot === 270).sort((a, b) => a.x - b.x);
const bottomIndex = new Map(bottoms.map((i, k) => [i, k]));
const made = [];
const L = 40;
for (const i of info) {
  const net = NET[i.name] || i.name;
  let ex = i.x, ey = i.y;
  if (i.rot === 180) { ex = i.x - L; ey = i.y; }
  else if (i.rot === 0) { ex = i.x + L; ey = i.y; }
  else if (i.rot === 270) { const k = bottomIndex.get(i); ex = i.x; ey = i.y - (L + k * 16); }
  await eda.sch_PrimitiveWire.create([i.x, i.y, ex, ey], net);
  await eda.sch_PrimitiveAttribute.createNetLabel(ex, ey, net);
  made.push({ name: i.name, net, at: [ex, ey] });
}
return { labeled: made.length, sample: made.slice(0, 6) };
