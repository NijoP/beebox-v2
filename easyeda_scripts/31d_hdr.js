const want = ["HDR-M_2.54_2x6P","HDR-M_2.54_1x6P","HDR-M 2.54 2X6","HDR-M 2.54 1X6"];
const out = {};
for (const q of want) {
  try { const r = await eda.lib_Device.search(q); out[q] = (r||[]).slice(0,3).map(d=>({name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid})); }
  catch (e) { out[q] = "ERR"; }
}
return out;
