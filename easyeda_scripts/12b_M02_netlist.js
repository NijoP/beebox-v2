let nl = "";
try { nl = await eda.sch_Netlist.getNetlist("Protel2"); } catch (e) { nl = "ERR:" + e; }
const lines = (nl || "").split(/\r?\n/);
const wires = await eda.sch_PrimitiveWire.getAllPrimitiveId();
const nets = [];
let cur = null;
for (const ln of lines) {
  const t = ln.trim();
  if (t === "(") { cur = { pins: [] }; }
  else if (t === ")") { if (cur) { nets.push(cur); cur = null; } }
  else if (cur) { if (!cur.name) cur.name = t; else cur.pins.push(t); }
}
const named = nets.filter(n => n.name && !/^NetR|^Net\d|^\$/.test(n.name));
return {
  netlistChars: (nl || "").length,
  wireCount: (wires || []).length,
  totalBlocks: nets.length,
  nets: nets.map(n => ({ net: n.name, pins: n.pins.length })).slice(0, 60)
};
