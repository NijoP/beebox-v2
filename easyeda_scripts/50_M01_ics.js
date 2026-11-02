const LIB = "0819f05c4eef4c71ace90d822a990e87";
const Lh = 100, Lv = 70;
async function label(compId, map) {
  const pins = await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(compId);
  for (const p of pins) {
    const nm = p.getState_PinName();
    const net = map[nm];
    if (net === undefined || net === null || net === "") continue;
    const x = p.getState_X(), y = p.getState_Y(), rot = p.getState_Rotation();
    let ex = x, ey = y, lrot = 0;
    if (rot === 180) { ex = x - Lh; }
    else if (rot === 0) { ex = x + Lh; }
    else if (rot === 270) { ey = y - Lv; lrot = 90; }
    else { ey = y + Lv; lrot = 90; }
    await eda.sch_PrimitiveWire.create([x, y, ex, ey], net);
    const lb = await eda.sch_PrimitiveAttribute.createNetLabel(ex, ey, net);
    try { await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(), { rotation: lrot, fontSize: 6 }); } catch (e) {}
  }
}
async function place(uuid, x, y) {
  const c = await eda.sch_PrimitiveComponent.create({ libraryUuid: LIB, uuid }, x, y, "", 0, false, true, true);
  return c.getState_PrimitiveId();
}
const bq = await place("a113361897bf47368d2912842c826e38", 500, 1600);
await label(bq, { "TS":"BQ_TS","BAT":"VBAT","CE#":"GND","EN2":"CHG_EN2","EN1":"CHG_EN1","PGOOD#":"CHG_PGOOD_N","VSS":"GND","CHG#":"CHG_STAT_N","OUT":"VSYS","ILIM":"BQ_ILIM","IN":"VBUS","TMR":"BQ_TMR","ITERM":"GND","ISET":"BQ_ISET","EP":"GND" });
const u3 = await place("009be0afb86044d8a62900f07f64ed07", 500, 1050);
await label(u3, { "EN":"VSYS","MODE":"REG_PS_SYNC","AGND":"GND","FB":"FB_3V3","PG":"REG_PG","VOUT":"+3V3","L2":"L_3V3B","GND":"GND","L1":"L_3V3A","VIN":"VSYS" });
const u4 = await place("62fc6c9d2a8e4c7aa120c7b71a097845", 950, 1050);
await label(u4, { "FB":"FB_5V0","EN":"VSYS","VIN":"VSYS","GND":"GND","SW":"L_5V0","VOUT":"+5V0" });
const u5 = await place("921d0fc4bb4347dca0bd09d3e42f87c6", 1200, 750);
await label(u5, { "IN":"+5V0","GND":"GND","EN":"CHAIN_PWR_EN","FAULT#":"CHAIN_FAULT_N","ILIM":"TPS2553_ILIM","OUT":"+5V_CHAIN" });
const jusb = await place("ea8fe62438f94521b482dce770be0817", 200, 2050);
await label(jusb, { "GND":"GND","VBUS":"VBUS","CC2":"CC2","DP2":"USB_DP","DN2":"USB_DM","SBU2":"SBU2","CC1":"CC1","DP1":"USB_DP","DN1":"USB_DM","SBU1":"SBU1","EH":"GND" });
const dusb = await place("ef0c95266254481fa44a3c27c326d03c", 800, 2050);
await label(dusb, { "IO1":"USB_DP","GND":"GND","IO2":"USB_DM" });
const jbat = await place("e36262a38c7e4ab8a1efe0df793ebdea", 1100, 2050);
await label(jbat, { "1":"VBAT","2":"GND","3":"GND","4":"GND" });
return { bq, u3, u4, u5, jusb, dusb, jbat };
