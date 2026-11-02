const out={};
for(const q of ["BAT54WS","B5819W SL","SS14"]){
 const r=await eda.lib_Device.search(q); if(!r||!r.length){out[q]="none";continue;} const d=r[0];
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:d.libraryUuid,uuid:d.uuid},5000,5000,"",0,false,true,true);
 const pins=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName()+"@"+p.getState_X());
 await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
 out[q]={name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid,pins};
}
return out;
