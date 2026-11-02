const nl = await eda.sch_Netlist.getNetlist("Protel2");
const lines = (nl || "").split(/\r?\n/);
const nets = []; let cur = null;
for (const ln of lines) {
  const t = ln.trim();
  if (t === "(") cur = { pins: [] };
  else if (t === ")") { if (cur) nets.push(cur); cur = null; }
  else if (cur) { if (!cur.name) cur.name = t; else cur.pins.push(t); }
}
const comps = await eda.sch_PrimitiveComponent.getAllPrimitiveId();
const wires = await eda.sch_PrimitiveWire.getAllPrimitiveId();
const key = {};
for (const n of nets) key[n.name] = n.pins;
const watch = ["+3V3","GND","MCU_EN","MCU_BOOT"];
const report = watch.map(w => ({ net: w, pinCount: (key[w]||[]).length, pins: key[w]||[] }));
const singletons = nets.filter(n => n.pins.length === 1).map(n => n.name);
return { components: comps.length, wires: wires.length, totalNets: nets.length, powerNets: report, singletonCount: singletons.length };
