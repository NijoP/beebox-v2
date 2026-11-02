const want = [
  "TPD2E2U06DRLR", "BQ24074RGTR", "TPS63802DLAR", "TPS61023DRLR", "TPS2553DBVR",
  "TPS22918DBVR", "TPD4E05U06DQAR", "USB4105-GF-A", "FH34SRJ-12S-0.5SH",
  "S2B-PH-SM4-TB", "B2B-PH-K-S", "TPS2553DBVR-1"
];
const out = {};
for (const q of want) {
  try {
    const r = await eda.lib_Device.search(q);
    const arr = (r || []).slice(0, 3).map(d => ({ name: d.name, mfr: d.manufacturerId, lcsc: d.supplierId, fp: d.footprintName, dev: d.uuid, lib: d.libraryUuid }));
    out[q] = arr;
  } catch (e) { out[q] = "ERR:" + e; }
}
return out;
