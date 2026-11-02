const c=await eda.sch_PrimitiveComponent.create({libraryUuid:"0819f05c4eef4c71ace90d822a990e87",uuid:"015cbd77079d4b0ea5c81b79fefdcea3"},500,3500,"",0,false,true,true);
const pins=await c.getAllPins();
const r=pins.map(p=>({n:p.getState_PinNumber(),nm:p.getState_PinName(),x:p.getState_X()}));
await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
return r;
