const LIB="0819f05c4eef4c71ace90d822a990e87";
async function lbl(id,map){
 const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);
 const g={}; for(const p of pins){const r=""+p.getState_Rotation();(g[r]=g[r]||[]).push(p);}
 for(const r of Object.keys(g)){const arr=g[r];
  if(r==="180"||r==="0")arr.sort((a,b)=>b.getState_Y()-a.getState_Y()); else arr.sort((a,b)=>a.getState_X()-b.getState_X());
  for(let k=0;k<arr.length;k++){const p=arr[k];const nm=p.getState_PinName();const net=map[nm];
   if(net==null||net==="")continue;const x=p.getState_X(),y=p.getState_Y(),R=+r,st=(k%2)*45;
   let ex=x,ey=y,lr=0;
   if(R===180)ex=x-(90+st); else if(R===0)ex=x+(90+st); else if(R===270){ey=y-(70+st);lr=90;} else {ey=y+(70+st);lr=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);
   const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);
   try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:lr,fontSize:6});}catch(e){}}}
}
async function p2(uuid,x,y,na,nb){
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid},x,y,"",0,false,true,true);
 const pins=await c.getAllPins();const pi=pins.map(p=>({x:p.getState_X(),y:p.getState_Y()}));
 const xs=[...new Set(pi.map(p=>p.x))].sort((a,b)=>a-b);
 for(const p of pi){const isL=p.x===xs[0];const net=isL?na:nb;const ex=isL?p.x-100:p.x+55;
  await eda.sch_PrimitiveWire.create([p.x,p.y,ex,p.y],net);
  const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,p.y,net);
  try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{fontSize:5});}catch(e){}}
}
const NET={"GND":"GND","3V3":"+3V3","EN":"MCU_EN","IO0":"MCU_BOOT","IO1":"SENSOR_DATA_ID","IO2":"VBAT_SENSE","IO4":"SENSOR_CLK","IO5":"SENSOR_SYNC","IO6":"SENSOR_PU_EN","IO7":"CHAIN_PWR_EN","IO8":"CHAIN_FAULT_N","IO9":"SYS_SDA","IO10":"SYS_SCL","IO11":"PATCH_SHIFT_DATA","IO12":"PATCH_SHIFT_CLK","IO13":"PATCH_LATCH","IO14":"PATCH_OE_N","IO15":"DISP_PWR_EN","IO16":"REG_PS_SYNC","IO17":"CHG_EN1","IO18":"CHG_EN2","IO19":"USB_DM","IO20":"USB_DP","IO21":"CHG_STAT_N","IO35":"CHG_PGOOD_N","IO36":"REG_PG","IO37":"GAUGE_ALRT_N","IO39":"LED_R","IO40":"LED_G","IO41":"LED_B","TXD0":"UART0_TX","RXD0":"UART0_RX","IO47":"PWR_HOLD","IO48":"PWR_BTN"};
const u1=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:"66b3838809bd4c87b584f0ab36e52b0a"},2000,2400,"",0,false,true,true);
await lbl(u1.getState_PrimitiveId(),NET);
// decoupling + boot/reset row above U1
await p2("9583d53cf848486a9d523fcaf153a729",1700,2760,"+3V3","GND");
await p2("2eaf9ba597d94b93917d2b625402739a",1950,2760,"+3V3","GND");
await p2("2eaf9ba597d94b93917d2b625402739a",2200,2760,"+3V3","GND");
await p2("ffb8c4ed7d7244bc918e37313d4ca373",2450,2760,"+3V3","MCU_EN");
await p2("115c3169541a48b3ba9c88523dfbcdee",2700,2760,"MCU_EN","GND");
await p2("ffb8c4ed7d7244bc918e37313d4ca373",1700,2620,"+3V3","MCU_BOOT");
await p2("5da69c8ac73640e59896b82d2fd17bab",1950,2620,"MCU_BOOT","GND");
await p2("38630614a58f4650bd5eefe9a5f79d9a",2250,2620,"MCU_EN","GND");
await p2("38630614a58f4650bd5eefe9a5f79d9a",2550,2620,"MCU_BOOT","GND");
return { u1: u1.getState_PrimitiveId() };
