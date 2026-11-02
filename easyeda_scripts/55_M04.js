const LIB = "0819f05c4eef4c71ace90d822a990e87";
const Lh=100, Lv=70;
async function label(compId, map){
  const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(compId);
  for(const p of pins){const nm=p.getState_PinName();const net=map[nm];
   if(net===undefined||net===null||net==="")continue;
   const x=p.getState_X(),y=p.getState_Y(),rot=p.getState_Rotation();
   let ex=x,ey=y,lrot=0;
   if(rot===180)ex=x-Lh; else if(rot===0)ex=x+Lh; else if(rot===270){ey=y-Lv;lrot=90;} else {ey=y+Lv;lrot=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);
   const lb=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);
   try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{rotation:lrot,fontSize:5});}catch(e){}}
}
async function place(uuid,x,y){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid},x,y,"",0,false,true,true);return c.getState_PrimitiveId();}
const CHAIN={"1":"GND","2":"+5V_CHAIN","3":"GND","4":"SENSOR_CLK_C","5":"SENSOR_SYNC_C","6":"GND","7":"SENSOR_DATA_ID","8":"PATCH_DATA_C","9":"PATCH_CLK_C","10":"PATCH_LATCH_C","11":"PATCH_OE_N_C","12":"AUX_ARM","13":"GND","14":"GND"};
const fpc=await place("56d0813866664aa693617cc16753ea47",2350,1350);
await label(fpc, CHAIN);
const pogo=await place("677932d4aced4f289d0065197a57e940",2350,850);
await label(pogo, CHAIN);
const esd1=await place("71bd5434893a460c8be823067c2f4e47",1950,1450);
await label(esd1, {"D1+":"SENSOR_CLK_C","D1-":"SENSOR_SYNC_C","D2+":"SENSOR_DATA_ID","D2-":"PATCH_DATA_C","GND":"GND"});
const esd2=await place("71bd5434893a460c8be823067c2f4e47",1950,1150);
await label(esd2, {"D1+":"PATCH_CLK_C","D1-":"PATCH_LATCH_C","D2+":"PATCH_OE_N_C","D2-":"AUX_ARM","GND":"GND"});
return { fpc, pogo, esd1, esd2 };
