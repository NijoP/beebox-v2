const wires = await eda.sch_PrimitiveWire.getAllPrimitiveId();
if (wires && wires.length) await eda.sch_PrimitiveWire.delete(wires);
const comps = await eda.sch_PrimitiveComponent.getAllPrimitiveId();
if (comps && comps.length) await eda.sch_PrimitiveComponent.delete(comps);
const attrsLeft = await eda.sch_PrimitiveAttribute.getAllPrimitiveId();
const wiresLeft = await eda.sch_PrimitiveWire.getAllPrimitiveId();
const compsLeft = await eda.sch_PrimitiveComponent.getAllPrimitiveId();
return {
  wiresDeleted: (wires || []).length,
  compsDeleted: (comps || []).length,
  attrsLeft: (attrsLeft || []).length,
  wiresLeft: (wiresLeft || []).length,
  compsLeft: (compsLeft || []).length
};
