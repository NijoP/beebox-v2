await eda.dmt_EditorControl.openDocument("3d113e2c2213fddc"); await new Promise(r=>setTimeout(r,1500));
const L="0819f05c4eef4c71ace90d822a990e87";const o={};
for(const [k,u] of [["CR2032","6c6537e92f8045d7b2cdb8b0ea44de27"],["RV3028","4c439d145251422d86fe12a14a648115"]]){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:L,uuid:u},6000,6000,"",0,false,true,true);
 o[k]=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName()).join(" ");await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);}
const d=await eda.dmt_SelectControl.getCurrentDocumentInfo(); o.doc=d.uuid; return o;
