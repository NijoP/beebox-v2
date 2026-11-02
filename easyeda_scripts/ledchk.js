const LIB="0819f05c4eef4c71ace90d822a990e87";
const cols={R:"015cbd77079d4b0ea5c81b79fefdcea3",G:"e8b07cfe9d254e27bcc3c1116be11e2b",B:"1a0921f6f61a426b8b7f0aa3c5392e45"};
const out={};
let yy=3600;
for(const k of Object.keys(cols)){
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:cols[k]},500,yy,"",0,false,true,true);
 const pins=await c.getAllPins();
 out[k]=pins.map(p=>({n:p.getState_PinNumber(),nm:p.getState_PinName(),x:p.getState_X()})).sort((a,b)=>a.x-b.x);
 await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);
 yy+=60;
}
return out;
