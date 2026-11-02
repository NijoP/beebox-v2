const out={};
for(const q of ["BAT54C","AO3400A"]){
 const r=await eda.lib_Device.search(q); const d=(r||[]).find(x=>x.name===q)||r[0];
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:d.libraryUuid,uuid:d.uuid},5000,5000,"",0,false,true,true);
 const pins=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName());
 await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
 out[q]={name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid,mfr:d.manufacturer,pins};
}
return out;
