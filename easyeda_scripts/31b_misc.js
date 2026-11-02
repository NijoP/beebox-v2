const want = ["0402WGF5623TCE","0402WGF7503TCE","10uF 0805 X5R","2.2uH 2A SMD","1uH 3A SMD","KT-0603R","Header 1x4 2.54","Header 1x12 2.54","Header 1x6 2.54"];
const out = {};
for (const q of want) {
  try { const r = await eda.lib_Device.search(q); out[q] = (r||[]).slice(0,2).map(d=>({name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid})); }
  catch (e) { out[q] = "ERR"; }
}
return out;
