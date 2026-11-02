const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const area=cs.filter(c=>c.getState_X()>=1750&&c.getState_X()<=2750&&c.getState_Y()>=1450&&c.getState_Y()<=1900).map(c=>({id:c.getState_PrimitiveId(),x:c.getState_X(),y:c.getState_Y()}));
return {total:cs.length,area};
