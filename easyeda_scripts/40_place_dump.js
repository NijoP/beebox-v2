const LIB = "0819f05c4eef4c71ace90d822a990e87";
const PARTS = [
  ["BQ24074", "a113361897bf47368d2912842c826e38", 650, 1500],
  ["TPS63802", "009be0afb86044d8a62900f07f64ed07", 650, 1150],
  ["TPS61023", "62fc6c9d2a8e4c7aa120c7b71a097845", 980, 1500],
  ["TPS2553", "921d0fc4bb4347dca0bd09d3e42f87c6", 980, 1150],
  ["USBC", "ea8fe62438f94521b482dce770be0817", 300, 1500],
  ["USBESD", "ef0c95266254481fa44a3c27c326d03c", 300, 1200],
  ["BATT", "e36262a38c7e4ab8a1efe0df793ebdea", 300, 1800],
  ["FPC", "56d0813866664aa693617cc16753ea47", 2250, 1300],
  ["POGO", "677932d4aced4f289d0065197a57e940", 2250, 850],
  ["CHESD1", "71bd5434893a460c8be823067c2f4e47", 1950, 1350],
  ["CHESD2", "71bd5434893a460c8be823067c2f4e47", 1950, 1050],
  ["DISP", "48c3a4f6cdc941c7b2c8ede78c08955e", 1350, 600],
  ["DEBUG", "677932d4aced4f289d0065197a57e940", 1750, 600]
];
const res = [];
for (const [key, uuid, x, y] of PARTS) {
  const c = await eda.sch_PrimitiveComponent.create({ libraryUuid: LIB, uuid }, x, y, "", 0, false, true, true);
  const pins = await c.getAllPins();
  const p = pins.map(pp => ({ n: pp.getState_PinNumber(), nm: pp.getState_PinName(), x: pp.getState_X(), y: pp.getState_Y(), r: pp.getState_Rotation() }));
  res.push({ key, id: c.getState_PrimitiveId(), pins: p });
}
return res;
