const want = [
  "0402WGF1001TCE", "0402WGF2201TCE", "0402WGF1003TCE", "0402WGF3303TCE",
  "0402WGF5623TCE", "0402WGF7503TCE", "0402WGF7682TCE", "0402WGF1004TCE",
  "0402WGF330JTCE", "0402WGF3300TCE",
  "CL21A106KOQNNNE", "0402B102K500NT",
  "FH34SRJ", "1.5uH 2A", "1uH 2A", "KT-0603R", "KT-0603F",
  "PinHeader 1x4 2.54", "PinHeader 1x3 2.54", "Header 2x6 2.54"
];
const out = {};
for (const q of want) {
  try {
    const r = await eda.lib_Device.search(q);
    out[q] = (r || []).slice(0, 2).map(d => ({ name: d.name, lcsc: d.supplierId, fp: d.footprintName, dev: d.uuid }));
  } catch (e) { out[q] = "ERR:" + e; }
}
return out;
