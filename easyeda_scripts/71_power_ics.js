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
async function place(uuid,x,y){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid},x,y,"",0,false,true,true);return c.getState_PrimitiveId();}
const usb=await place("ea8fe62438f94521b482dce770be0817",350,2350);
await lbl(usb,{"GND":"GND","VBUS":"VBUS","CC2":"CC2","DP2":"USB_DP","DN2":"USB_DM","SBU2":"SBU2","CC1":"CC1","DP1":"USB_DP","DN1":"USB_DM","SBU1":"SBU1","EH":"GND"});
const bq=await place("a113361897bf47368d2912842c826e38",950,2350);
await lbl(bq,{"TS":"BQ_TS","BAT":"VBAT","CE#":"GND","EN2":"CHG_EN2","EN1":"CHG_EN1","PGOOD#":"CHG_PGOOD_N","VSS":"GND","CHG#":"CHG_STAT_N","OUT":"VSYS","ILIM":"BQ_ILIM","IN":"VBUS","TMR":"BQ_TMR","ITERM":"GND","ISET":"BQ_ISET","EP":"GND"});
const u3=await place("009be0afb86044d8a62900f07f64ed07",400,1750);
await lbl(u3,{"EN":"PWR_EN","MODE":"REG_PS_SYNC","AGND":"GND","FB":"FB_3V3","PG":"REG_PG","VOUT":"+3V3","L2":"L_3V3B","GND":"GND","L1":"L_3V3A","VIN":"VSYS"});
const u4=await place("62fc6c9d2a8e4c7aa120c7b71a097845",950,1750);
await lbl(u4,{"FB":"FB_5V0","EN":"PWR_EN","VIN":"VSYS","GND":"GND","SW":"L_5V0","VOUT":"+5V0"});
const u5=await place("921d0fc4bb4347dca0bd09d3e42f87c6",1350,1750);
await lbl(u5,{"IN":"+5V0","GND":"GND","EN":"CHAIN_PWR_EN","FAULT#":"CHAIN_FAULT_N","ILIM":"TPS2553_ILIM","OUT":"+5V_CHAIN"});
const esd=await place("ef0c95266254481fa44a3c27c326d03c",350,2850);
await lbl(esd,{"IO1":"USB_DP","GND":"GND","IO2":"USB_DM"});
const bat=await place("e36262a38c7e4ab8a1efe0df793ebdea",750,2850);
await lbl(bat,{"1":"VBAT","2":"GND","3":"GND","4":"GND"});
return { usb, bq, u3, u4, u5, esd, bat };
