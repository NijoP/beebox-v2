const r=await eda.lib_Device.search("SN74LVC1G08DBVR");const d=(r||[]).find(x=>x.name==="SN74LVC1G08DBVR")||r[0];
const c=await eda.sch_PrimitiveComponent.create({libraryUuid:d.libraryUuid,uuid:d.uuid},6000,6000,"",0,false,true,true);
const pins=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName()).join(" ");await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
return [d.name,d.supplierId,d.footprintName,d.uuid,pins];
