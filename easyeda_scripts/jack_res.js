await eda.dmt_EditorControl.openDocument("3d113e2c2213fddc"); await new Promise(r=>setTimeout(r,1500));
const out={};
for(const q of ["DC-005-5A-2.0","DC-005","SMD1812P200TF/33"]){const r=await eda.lib_Device.search(q);const d=(r||[]).find(x=>x.name===q)||(r||[])[0];if(!d){out[q]="none";continue;}
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:d.libraryUuid,uuid:d.uuid},6000,6000,"",0,false,true,true);
 out[q]=[d.name,d.supplierId,d.footprintName,d.uuid,(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName()).join(" ")];
 await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);}
return out;
