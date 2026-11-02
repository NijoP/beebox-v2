const LIB="0819f05c4eef4c71ace90d822a990e87";
const P=[["BATT_S2B","e36262a38c7e4ab8a1efe0df793ebdea"],["FPC_FH34","56d0813866664aa693617cc16753ea47"],["HDR_2x6","677932d4aced4f289d0065197a57e940"],["HDR_1x4","48c3a4f6cdc941c7b2c8ede78c08955e"]];
const res=[]; let yy=4000;
for(const [k,u] of P){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:u},500,yy,"",0,false,true,true);const pins=await c.getAllPins();res.push({key:k,pins:pins.map(p=>p.getState_PinNumber()+":"+p.getState_PinName())});await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);yy+=80;}
return res;
