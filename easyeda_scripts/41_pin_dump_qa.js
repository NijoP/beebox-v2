const LIB = "0819f05c4eef4c71ace90d822a990e87";
const PARTS = [
  ["BQ24074", "a113361897bf47368d2912842c826e38"],
  ["TPS63802", "009be0afb86044d8a62900f07f64ed07"],
  ["TPS61023", "62fc6c9d2a8e4c7aa120c7b71a097845"],
  ["TPS2553-1", "921d0fc4bb4347dca0bd09d3e42f87c6"],
  ["USBC", "ea8fe62438f94521b482dce770be0817"],
  ["USBESD_TPD2E2U06", "ef0c95266254481fa44a3c27c326d03c"],
  ["CHESD_TPD4E05U06", "71bd5434893a460c8be823067c2f4e47"],
  ["SHT45", "d2ae2b1001894b88873ae8d8ab95ec2d"],
  ["ICS43434", "ea6d3dad996d464d91983d780e0fb112"],
  ["BATT_S2B_PH", "e36262a38c7e4ab8a1efe0df793ebdea"],
  ["FPC_FH34", "56d0813866664aa693617cc16753ea47"],
  ["HDR_2x6", "677932d4aced4f289d0065197a57e940"],
  ["HDR_1x4", "48c3a4f6cdc941c7b2c8ede78c08955e"]
];
const res = [];
let yy = 4000;
for (const [key, uuid] of PARTS) {
  const c = await eda.sch_PrimitiveComponent.create({ libraryUuid: LIB, uuid }, 500, yy, "", 0, false, true, true);
  const pins = await c.getAllPins();
  const p = pins.map(pp => ({ n: pp.getState_PinNumber(), nm: pp.getState_PinName() }));
  res.push({ key, pinCount: p.length, pins: p });
  await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
  yy += 100;
}
return res;
