const L="0819f05c4eef4c71ace90d822a990e87";const out={};
for(const [k,u] of [["LMR51420","9f7005c05c264d3aa9ef2db611895932"],["XH","4d018698282b47d4893c87aca1c32f67"],["PTC","6800a564dc6b4d7a98c275222893ecd6"],["SS34","b8a5c6cf81fb4f7e8e47029a0abe40d8"],["SMBJ","fadafc8c52304b4fbd6862ec903a02ce"]]){
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:L,uuid:u},5000,5000,"",0,false,true,true);
 out[k]=(await c.getAllPins()).map(p=>p.getState_PinNumber()+":"+p.getState_PinName()+"@"+(p.getState_X()-5000)).join(" ");
 await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);}
return out;
