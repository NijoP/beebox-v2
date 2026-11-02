const pins = await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId("ce815dd3f4fbab2f");
return pins.map(p=>({n:p.getState_PinNumber(),x:p.getState_X(),y:p.getState_Y(),r:p.getState_Rotation()})).sort((a,b)=>(+a.n)-(+b.n));
