const device = { libraryUuid: "0819f05c4eef4c71ace90d822a990e87", uuid: "66b3838809bd4c87b584f0ab36e52b0a" };
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
// Net labels render with text extending to the RIGHT of the anchor point.
// So: left pins anchor far-left (text fills the clear gap toward the chip, stops short of pin numbers);
// right pins anchor at the outer stub end (text extends into open space);
// bottom pins use a long stub with the label low + rotated 90 (clear of the chip-edge pin names).
const comp = await eda.sch_PrimitiveComponent.create(device, 1500, 1200, "", 0, false, true, true);
const compId = comp.getState_PrimitiveId();
const pins = await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(compId);
let n = 0;
for (const p of pins) {
  const name = p.getState_PinName();
  const net = NET[name] || name;
  const x = p.getState_X(), y = p.getState_Y(), rot = p.getState_Rotation();
  let ex = x, ey = y, lrot = 0, lx = x, ly = y, fs = 6;
  if (rot === 180) { ex = x - 130; ey = y; lx = ex; ly = y; lrot = 0; }
  else if (rot === 0) { ex = x + 70; ey = y; lx = ex; ly = y; lrot = 0; }
  else { ex = x; ey = y - 130; lx = x; ly = y - 105; lrot = 90; fs = 5; }
  await eda.sch_PrimitiveWire.create([x, y, ex, ey], net);
  const lbl = await eda.sch_PrimitiveAttribute.createNetLabel(lx, ly, net);
  try { await eda.sch_PrimitiveAttribute.modify(lbl.getState_PrimitiveId(), { rotation: lrot, fontSize: fs }); } catch (e) { }
  n++;
}
return { compId, cx: comp.getState_X(), cy: comp.getState_Y(), labeled: n };
