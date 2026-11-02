const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"wrong sheet",DOC};
const w=await eda.sch_PrimitiveWire.getAllPrimitiveId(); if(w.length) await eda.sch_PrimitiveWire.delete(w);
const comps=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const del=comps.filter(c=>c.getState_ComponentType()!=="sheet").map(c=>c.getState_PrimitiveId());
if(del.length) await eda.sch_PrimitiveComponent.delete(del);
const tids=await eda.sch_PrimitiveText.getAllPrimitiveId(); if(tids&&tids.length) await eda.sch_PrimitiveText.delete(tids);
return {wires:w.length,comps:del.length,texts:(tids||[]).length,left:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length,attrs:(await eda.sch_PrimitiveAttribute.getAllPrimitiveId()).length};
