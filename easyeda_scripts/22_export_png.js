let f;
try { f = await eda.sch_ManufactureData.getPngFile("m02", { width: 2600 }); }
catch (e) { try { f = await eda.sch_ManufactureData.getPngFile("m02"); } catch (e2) { return { error: String(e2) }; } }
if (!f) return { error: "no file returned" };
const buf = await f.arrayBuffer();
const bytes = new Uint8Array(buf);
let bin = "";
const chunk = 0x8000;
for (let i = 0; i < bytes.length; i += chunk) { bin += String.fromCharCode.apply(null, bytes.subarray(i, i + chunk)); }
return { name: f.name, size: bytes.length, b64: btoa(bin) };
