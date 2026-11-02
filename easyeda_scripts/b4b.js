const r=await eda.lib_Device.search("B4B-PH-K-S(LF)(SN)");
const d=r[0];const c=await eda.sch_PrimitiveComponent.create({libraryUuid:d.libraryUuid,uuid:d.uuid},3000,3000,"",0,false,true,true);
const pins=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName());
await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
return {name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid,pins};
