const LIB = "0819f05c4eef4c71ace90d822a990e87";
const D = {
  c22u: "9583d53cf848486a9d523fcaf153a729", c100n: "2eaf9ba597d94b93917d2b625402739a",
  c1u: "115c3169541a48b3ba9c88523dfbcdee", c10n: "5da69c8ac73640e59896b82d2fd17bab",
  r10k: "ffb8c4ed7d7244bc918e37313d4ca373", sw: "38630614a58f4650bd5eefe9a5f79d9a"
};
// placed in a clear band ABOVE U1 (U1 labels occupy y 970..1300). Rows at y=1460 and y=1340.
const PLAN = [
  ["c22u", 1300, 1460, "+3V3", "GND"],
  ["c100n", 1540, 1460, "+3V3", "GND"],
  ["c100n", 1780, 1460, "+3V3", "GND"],
  ["r10k", 2020, 1460, "+3V3", "MCU_EN"],
  ["c1u", 2260, 1460, "MCU_EN", "GND"],
  ["r10k", 1300, 1340, "+3V3", "MCU_BOOT"],
  ["c10n", 1540, 1340, "MCU_BOOT", "GND"],
  ["sw", 1820, 1340, "MCU_EN", "GND"],
  ["sw", 2100, 1340, "MCU_BOOT", "GND"]
];
const made = [];
for (const [key, x, y, netA, netB] of PLAN) {
  const comp = await eda.sch_PrimitiveComponent.create({ libraryUuid: LIB, uuid: D[key] }, x, y, "", 0, false, true, true);
  const pins = await comp.getAllPins();
  const pi = pins.map(p => ({ x: p.getState_X(), y: p.getState_Y() }));
  const xs = [...new Set(pi.map(p => p.x))].sort((a, b) => a - b);
  const horizontal = xs.length >= 2;
  for (const p of pi) {
    let net, wx, lx;
    if (horizontal) {
      const isLeft = p.x === xs[0];
      net = isLeft ? netA : netB;
      wx = isLeft ? p.x - 55 : p.x + 55;       // stub end
      lx = isLeft ? p.x - 55 : p.x + 10;       // label anchor (text extends right)
      await eda.sch_PrimitiveWire.create([p.x, p.y, wx, p.y], net);
      const lbl = await eda.sch_PrimitiveAttribute.createNetLabel(lx, p.y, net);
      try { await eda.sch_PrimitiveAttribute.modify(lbl.getState_PrimitiveId(), { fontSize: 6 }); } catch (e) {}
    } else {
      const ys = [...new Set(pi.map(q => q.y))].sort((a, b) => a - b);
      const isBottom = p.y === ys[0];
      net = isBottom ? netB : netA;
      const wy = isBottom ? p.y - 45 : p.y + 45;
      await eda.sch_PrimitiveWire.create([p.x, p.y, p.x, wy], net);
      const lbl = await eda.sch_PrimitiveAttribute.createNetLabel(p.x, wy, net);
      try { await eda.sch_PrimitiveAttribute.modify(lbl.getState_PrimitiveId(), { fontSize: 6 }); } catch (e) {}
    }
  }
  made.push({ key, netA, netB });
}
return { placed: made.length };
