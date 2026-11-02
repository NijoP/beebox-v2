const want=["LMR51420XDDCR","B2B-XH-A(LF)(SN)","SMD1812P110TF/33","SS34","SMBJ26A","CL31A106KBHNNNE","0402WGF1372TCE","SWPA6045S100MT","CL05B104KB54PNC"];
const out={};
for(const q of want){try{const r=await eda.lib_Device.search(q);const d=(r||[]).find(x=>x.name===q)||(r||[])[0];if(!d){out[q]="none";continue;}
 let pins="";try{const c=await eda.sch_PrimitiveComponent.create({libraryUuid:d.libraryUuid,uuid:d.uuid},5000,5000,"",0,false,true,true);pins=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName()+"@"+(p.getState_X()-5000)).join(" ");await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);}catch(e){pins="ERR"}
 out[q]=[d.name,d.supplierId,d.footprintName,d.uuid,pins];}catch(e){out[q]="ERR "+e;}}
return out;
