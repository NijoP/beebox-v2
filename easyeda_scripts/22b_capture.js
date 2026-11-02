const doc = await eda.dmt_SelectControl.getCurrentDocumentInfo();
const tabId = doc && doc.tabId;
try { await eda.dmt_EditorControl.zoomToAllPrimitives(tabId); } catch (e) { }
await new Promise(r => setTimeout(r, 600));
let blob;
try { blob = await eda.dmt_EditorControl.getCurrentRenderedAreaImage(tabId); }
catch (e) { return { error: String(e) }; }
if (!blob) return { error: "no blob" };
const buf = await blob.arrayBuffer();
const bytes = new Uint8Array(buf);
let bin = "";
const chunk = 0x8000;
for (let i = 0; i < bytes.length; i += chunk) { bin += String.fromCharCode.apply(null, bytes.subarray(i, i + chunk)); }
return { size: bytes.length, type: blob.type, b64: btoa(bin) };
