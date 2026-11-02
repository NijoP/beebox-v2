await eda.dmt_EditorControl.openDocument("7d488638e1dfc7e4"); await new Promise(r=>setTimeout(r,2500));
const d=await eda.dmt_SelectControl.getCurrentDocumentInfo();
const before=(await eda.pcb_PrimitiveComponent.getAllPrimitiveId()).length;
let ok=null,err=null; try{ ok=await eda.pcb_Document.importChanges(); }catch(e){err=String(e);}
await new Promise(r=>setTimeout(r,4000));
const ids=await eda.pcb_PrimitiveComponent.getAllPrimitiveId();
const comps=await eda.pcb_PrimitiveComponent.get(ids);
const refs=comps.map(c=>c.getState_Designator()).filter(Boolean);
return {doc:d.uuid,docType:d.documentType,before,ok,err,after:ids.length,mainN:refs.filter(r=>+(r.replace(/\D/g,''))<100).length,patchN:refs.filter(r=>{const n=+(r.replace(/\D/g,''));return n>=100&&n<200;}).length,splitN:refs.filter(r=>+(r.replace(/\D/g,''))>=200).length};
