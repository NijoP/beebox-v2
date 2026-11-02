const esd1="ca10109d231823b1";
const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(esd1);
const esdPins = pins.map(p=>({nm:p.getState_PinName(),x:p.getState_X(),y:p.getState_Y(),r:p.getState_Rotation()}));
// also locate R23: scan components for one near (2150,1700)
const ids=await eda.sch_PrimitiveComponent.getAllPrimitiveId();
const comps=await eda.sch_PrimitiveComponent.get(ids);
let r23=null;
for(const c of comps){ const x=c.getState_X(),y=c.getState_Y(); if(Math.abs(x-2150)<30&&Math.abs(y-1700)<30){ r23={id:c.getState_PrimitiveId(),x,y}; } }
let r23pins=null;
if(r23){ const rp=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(r23.id); r23pins=rp.map(p=>({x:p.getState_X(),y:p.getState_Y()})); }
return { esdPins, r23, r23pins };
