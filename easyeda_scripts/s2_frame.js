const ids=await eda.sch_PrimitiveComponent.getAllPrimitiveId();
const c=await eda.sch_PrimitiveComponent.get(ids);
return c.map(x=>({id:x.getState_PrimitiveId(),x:x.getState_X(),y:x.getState_Y(),type:x.getState_ComponentType?x.getState_ComponentType():"?"}));
