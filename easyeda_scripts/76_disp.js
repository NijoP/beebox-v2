const LIB="0819f05c4eef4c71ace90d822a990e87";
async function lbl(id,map){
 const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);
 const g={}; for(const p of pins){const r=""+p.getState_Rotation();(g[r]=g[r]||[]).push(p);}
 for(const r of Object.keys(g)){const arr=g[r];
  if(r==="180"||r==="0")arr.sort((a,b)=>b.getState_Y()-a.getState_Y()); else arr.sort((a,b)=>a.getState_X()-b.getState_X());
  for(let k=0;k<arr.length;k++){const p=arr[k];const nm=p.getState_PinName();const net=map[nm];
   if(net==null||net==="")continue;const x=p.getState_X(),y=p.getState_Y(),R=+r,st=(k%2)*60;
   let ex=x,ey=y,lr=0;
   if(R===180)ex=x-(110+st); else if(R===0)ex=x+(110+st); else if(R===270){ey=y-(80+st);lr=90;} else {ey=y+(80+st);lr=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);
   const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);
   try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:lr,fontSize:5});}catch(e){}}}
}
async function place(uuid,x,y){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid},x,y,"",0,false,true,true);return c.getState_PrimitiveId();}
const sht=await place("d2ae2b1001894b88873ae8d8ab95ec2d",1600,1550); await lbl(sht,{"SDA":"SYS_SDA","SCL":"SYS_SCL","VDD":"+3V3","VSS":"GND","5":"GND"});
const ics=await place("ea6d3dad996d464d91983d780e0fb112",2050,1550); await lbl(ics,{"WS":"SENSOR_SYNC","LR":"GND","GND":"GND","SCK":"SENSOR_CLK","VDD":"+3V3","SD":"SENSOR_DATA_ID"});
const disp=await place("48c3a4f6cdc941c7b2c8ede78c08955e",2500,1550); await lbl(disp,{"1":"GND","2":"+3V3","3":"SYS_SCL","4":"SYS_SDA"});
const dbg=await place("677932d4aced4f289d0065197a57e940",2950,1550); await lbl(dbg,{"1":"GND","2":"+3V3","3":"UART0_TX","4":"UART0_RX","5":"MCU_EN","6":"MCU_BOOT","7":"DISP_PWR_EN","8":"GAUGE_ALRT_N","9":"AUX_ARM","10":"IO42","11":"IO38","12":"GND"});
return {sht,ics,disp,dbg};
