const doc = await eda.dmt_SelectControl.getCurrentDocumentInfo();
const tabId = doc && doc.tabId;
try { await eda.dmt_EditorControl.zoomTo(1450, 1150, 0.5, tabId); } catch (e) {}
await new Promise(r => setTimeout(r, 400));
try { await eda.dmt_EditorControl.zoomToRegion(1250, 1780, 1330, 960, tabId); } catch (e) {}
await new Promise(r => setTimeout(r, 900));
const blob = await eda.dmt_EditorControl.getCurrentRenderedAreaImage(tabId);
if (!blob) return { error: "no blob" };
const buf = await blob.arrayBuffer();
const bytes = new Uint8Array(buf);
let bin = ""; const chunk = 0x8000;
for (let i = 0; i < bytes.length; i += chunk) bin += String.fromCharCode.apply(null, bytes.subarray(i, i + chunk));
return { size: bytes.length, b64: btoa(bin) };
