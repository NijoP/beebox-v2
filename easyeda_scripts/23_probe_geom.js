const wireIds = await eda.sch_PrimitiveWire.getAllPrimitiveId();
const wires = await eda.sch_PrimitiveWire.get(wireIds);
let minX = 1e9, maxX = -1e9, minY = 1e9;
const leftEnds = [];
for (const w of (wires || [])) {
  let pts = null;
  try { pts = w.getState_Line ? w.getState_Line() : null; } catch (e) {}
  if (!pts) continue;
  for (let i = 0; i < pts.length; i += 2) {
    const x = pts[i], y = pts[i + 1];
    if (x < minX) minX = x;
    if (x > maxX) maxX = x;
    if (y < minY) minY = y;
  }
}
return { wireCount: (wireIds || []).length, minX, maxX, minY, firstWireSample: (wires && wires[0] && wires[0].getState_Line) ? wires[0].getState_Line() : "no getState_Line" };
