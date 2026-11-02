const want = ["CL21A106KAQNNNE","CL21A106KOQNNNE","SWPA4030S1R5NT","SWPA4030S1R0NT","SWPA3015S1R5NT","1X4P-2_54","2X6P-2_54","KT-0603G","KT-0603B","HDR-M-2.54_1X4"];
const out = {};
for (const q of want) {
  try { const r = await eda.lib_Device.search(q); out[q] = (r||[]).slice(0,2).map(d=>({name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid})); }
  catch (e) { out[q] = "ERR"; }
}
return out;
