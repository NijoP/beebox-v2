const doc = await eda.dmt_SelectControl.getCurrentDocumentInfo();
if (!doc || doc.documentType !== 1) { return { error: "active doc is not a schematic page", doc }; }
const device = { libraryUuid: "0819f05c4eef4c71ace90d822a990e87", uuid: "66b3838809bd4c87b584f0ab36e52b0a" };
const comp = await eda.sch_PrimitiveComponent.create(device, 1500, 1200, "", 0, false, true, true);
const pins = await comp.getAllPins();
const out = [];
for (const p of pins) {
  out.push({ n: p.getState_PinNumber(), name: p.getState_PinName(), x: p.getState_X(), y: p.getState_Y(), rot: p.getState_Rotation() });
}
out.sort((a, b) => (parseInt(a.n) || 0) - (parseInt(b.n) || 0));
return { compId: comp.getState_PrimitiveId(), cx: comp.getState_X(), cy: comp.getState_Y(), pinCount: pins.length, pins: out };
