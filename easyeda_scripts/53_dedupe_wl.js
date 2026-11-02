// Dedupe overlapping wires
const wids = await eda.sch_PrimitiveWire.getAllPrimitiveId();
const wires = await eda.sch_PrimitiveWire.get(wids);
const seenW = {}; const delW = [];
for (const w of wires) {
  let ln = null; try { ln = w.getState_Line(); } catch(e){}
  const key = ln ? ln.join(",") : w.getState_PrimitiveId();
  if (seenW[key]) delW.push(w.getState_PrimitiveId()); else seenW[key] = 1;
}
if (delW.length) await eda.sch_PrimitiveWire.delete(delW);
// Dedupe overlapping net labels
const aids = await eda.sch_PrimitiveAttribute.getAllPrimitiveId();
const attrs = await eda.sch_PrimitiveAttribute.get(aids);
const seenA = {}; const delA = [];
for (const a of attrs) {
  let x,y,v; try { x=a.getState_X(); y=a.getState_Y(); v=a.getState_Value?a.getState_Value():(a.getState_Net?a.getState_Net():""); } catch(e){ continue; }
  const key = Math.round(x)+","+Math.round(y)+","+v;
  if (seenA[key]) delA.push(a.getState_PrimitiveId()); else seenA[key]=1;
}
let delAok = 0;
for (const id of delA) { try { await eda.sch_PrimitiveAttribute.delete([id]); delAok++; } catch(e){} }
return { wiresDeleted: delW.length, wiresLeft: (await eda.sch_PrimitiveWire.getAllPrimitiveId()).length, labelDupFound: delA.length, labelDeleted: delAok };
