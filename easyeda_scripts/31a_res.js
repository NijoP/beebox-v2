const want = ["0402WGF1001TCE","0402WGF2201TCE","0402WGF1003TCE","0402WGF7682TCE","0402WGF1004TCE","0402WGF3303TCE","0402WGF330JTCE","0402WGF5101TCE"];
const out = {};
for (const q of want) {
  try { const r = await eda.lib_Device.search(q); out[q] = (r||[]).slice(0,1).map(d=>({name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid})); }
  catch (e) { out[q] = "ERR"; }
}
return out;
