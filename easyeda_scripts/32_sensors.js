const want = ["SHT45-AD1B-R2","SHT45","ICS-43434","C5656610"];
const out = {};
for (const q of want) {
  try { const r = await eda.lib_Device.search(q); out[q] = (r||[]).slice(0,3).map(d=>({name:d.name,mfr:d.manufacturerId,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid})); }
  catch (e) { out[q] = "ERR"; }
}
return out;
