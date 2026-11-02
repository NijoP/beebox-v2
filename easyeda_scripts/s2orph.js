const box=(x,y)=>x>950&&x<2650&&y>-60&&y<760;
const attrs=await eda.sch_PrimitiveAttribute.get(await eda.sch_PrimitiveAttribute.getAllPrimitiveId());
const inbox=[];for(const a of attrs){let x,y,k,v;try{x=a.getState_X();y=a.getState_Y();k=a.getState_Key();v=a.getState_Value();}catch(e){continue;}if(box(x,y))inbox.push([k,v,x,y]);}
const comps=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
return {orphansInBox:inbox.length,sample:inbox.slice(0,5),comps:comps.map(c=>[c.getState_X(),c.getState_Y()]).filter(p=>p[1]<760)};
