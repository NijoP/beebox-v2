const LIB="0819f05c4eef4c71ace90d822a990e87";
const P=[["595","f7296b70af1e425f9d78faa598566f18"],["TPS62840","b902ba68e4fd4b048ff83be09f873cbc"],["TMUX1511","4ab6cbe4c9f94cc58a3871090025db9c"],["TMUX1109","44436dfd2ba34f6a9c82d12c2dcaafbb"],["TPS2552","d968c2971f5541f19281d883c8e84991"],["TPS22918","e30f49de94c341bea49e53f3016845f9"],["B5B","71bdd799d1af4500b01fdf95cb8f7869"]];
const res={};let yy=3000;
for(const[k,u]of P){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:u},3000,yy,"",0,false,true,true);const pins=await c.getAllPins();res[k]=pins.map(p=>p.getState_PinNumber()+":"+p.getState_PinName()).join(" ");await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);yy+=100;}
res.left=(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length;
return res;
