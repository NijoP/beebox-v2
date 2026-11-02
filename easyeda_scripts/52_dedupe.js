const ids = await eda.sch_PrimitiveComponent.getAllPrimitiveId();
const comps = await eda.sch_PrimitiveComponent.get(ids);
const byPos = {};
for (const c of comps) {
  const key = Math.round(c.getState_X()) + "," + Math.round(c.getState_Y());
  (byPos[key] = byPos[key] || []).push(c.getState_PrimitiveId());
}
const toDelete = [];
for (const key in byPos) {
  const arr = byPos[key];
  if (arr.length > 1) { for (let i = 1; i < arr.length; i++) toDelete.push(arr[i]); }
}
if (toDelete.length) await eda.sch_PrimitiveComponent.delete(toDelete);
const after = (await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length;
return { duplicatePositions: Object.values(byPos).filter(a=>a.length>1).length, deleted: toDelete.length, componentsAfter: after };
