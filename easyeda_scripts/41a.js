const LIB="0819f05c4eef4c71ace90d822a990e87";
const P=[["BQ24074","a113361897bf47368d2912842c826e38"],["TPS63802","009be0afb86044d8a62900f07f64ed07"],["TPS61023","62fc6c9d2a8e4c7aa120c7b71a097845"],["TPS2553-1","921d0fc4bb4347dca0bd09d3e42f87c6"]];
const res=[]; let yy=4000;
for(const [k,u] of P){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:u},500,yy,"",0,false,true,true);const pins=await c.getAllPins();res.push({key:k,pins:pins.map(p=>p.getState_PinNumber()+":"+p.getState_PinName())});await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);yy+=80;}
return res;
