const doc=await eda.dmt_SelectControl.getCurrentDocumentInfo();
await eda.dmt_EditorControl.zoomToRegion(50,2450,1750,0,doc.tabId);
await new Promise(r=>setTimeout(r,1500));
const blob=await eda.dmt_EditorControl.getCurrentRenderedAreaImage(doc.tabId);
const b=new Uint8Array(await blob.arrayBuffer());let s="";for(let i=0;i<b.length;i+=0x8000)s+=String.fromCharCode.apply(null,b.subarray(i,i+0x8000));
return {b64:btoa(s),size:b.length};
