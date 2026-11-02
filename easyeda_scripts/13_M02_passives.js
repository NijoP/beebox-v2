const LIB = "0819f05c4eef4c71ace90d822a990e87";
const D = {
  c22u: "9583d53cf848486a9d523fcaf153a729",
  c100n: "2eaf9ba597d94b93917d2b625402739a",
  c1u: "115c3169541a48b3ba9c88523dfbcdee",
  c10n: "5da69c8ac73640e59896b82d2fd17bab",
  r10k: "ffb8c4ed7d7244bc918e37313d4ca373",
  sw: "38630614a58f4650bd5eefe9a5f79d9a"
};
const PLAN = [
  ["c22u", 1150, 1450, "+3V3", "GND"],
  ["c100n", 1310, 1450, "+3V3", "GND"],
  ["c100n", 1470, 1450, "+3V3", "GND"],
  ["r10k", 1640, 1450, "+3V3", "MCU_EN"],
  ["c1u", 1810, 1450, "MCU_EN", "GND"],
  ["r10k", 1150, 1580, "+3V3", "MCU_BOOT"],
  ["c10n", 1310, 1580, "MCU_BOOT", "GND"],
  ["sw", 1480, 1580, "MCU_EN", "GND"],
  ["sw", 1650, 1580, "MCU_BOOT", "GND"]
];
const made = [];
const S = 20;
for (const [key, x, y, netA, netB] of PLAN) {
  const comp = await eda.sch_PrimitiveComponent.create({ libraryUuid: LIB, uuid: D[key] }, x, y, "", 0, false, true, true);
  const pins = await comp.getAllPins();
  const pi = pins.map(p => ({ x: p.getState_X(), y: p.getState_Y() }));
  const xs = [...new Set(pi.map(p => p.x))].sort((a, b) => a - b);
  const horizontal = xs.length >= 2;
  for (const p of pi) {
    let net, ex, ey;
    if (horizontal) {
      const isLeft = p.x === xs[0];
      net = isLeft ? netA : netB;
      ex = isLeft ? p.x - S : p.x + S; ey = p.y;
    } else {
      const ys = [...new Set(pi.map(q => q.y))].sort((a, b) => a - b);
      const isBottom = p.y === ys[0];
      net = isBottom ? netB : netA;
      ex = p.x; ey = isBottom ? p.y - S : p.y + S;
    }
    await eda.sch_PrimitiveWire.create([p.x, p.y, ex, ey], net);
    await eda.sch_PrimitiveAttribute.createNetLabel(ex, ey, net);
  }
  made.push({ key, ref: comp.getState_PrimitiveId(), pins: pi.length, netA, netB });
}
return { placed: made.length, parts: made };
