const LIB="0819f05c4eef4c71ace90d822a990e87";
const P=[["USBC","ea8fe62438f94521b482dce770be0817"],["USBESD","ef0c95266254481fa44a3c27c326d03c"],["CHESD","71bd5434893a460c8be823067c2f4e47"],["SHT45","d2ae2b1001894b88873ae8d8ab95ec2d"],["ICS43434","ea6d3dad996d464d91983d780e0fb112"]];
const res=[]; let yy=4000;
for(const [k,u] of P){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:u},500,yy,"",0,false,true,true);const pins=await c.getAllPins();res.push({key:k,pins:pins.map(p=>p.getState_PinNumber()+":"+p.getState_PinName())});await eda.sch_PrimitiveComponent.delete([c.getState_PrimitiveId()]);yy+=80;}
return res;
