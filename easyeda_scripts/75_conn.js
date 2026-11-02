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
const CHAIN={"1":"GND","2":"+5V_CHAIN","3":"GND","4":"SENSOR_CLK_C","5":"SENSOR_SYNC_C","6":"GND","7":"SENSOR_DATA_ID","8":"PATCH_DATA_C","9":"PATCH_CLK_C","10":"PATCH_LATCH_C","11":"PATCH_OE_N_C","12":"AUX_ARM","13":"GND","14":"GND"};
const fpc=await place("56d0813866664aa693617cc16753ea47",4100,2450); await lbl(fpc,CHAIN);
const pogo=await place("677932d4aced4f289d0065197a57e940",4100,1850); await lbl(pogo,CHAIN);
const e1=await place("71bd5434893a460c8be823067c2f4e47",3650,2550); await lbl(e1,{"D1+":"SENSOR_CLK_C","D1-":"SENSOR_SYNC_C","D2+":"SENSOR_DATA_ID","D2-":"PATCH_DATA_C","GND":"GND"});
const e2=await place("71bd5434893a460c8be823067c2f4e47",3650,2150); await lbl(e2,{"D1+":"PATCH_CLK_C","D1-":"PATCH_LATCH_C","D2+":"PATCH_OE_N_C","D2-":"AUX_ARM","GND":"GND"});
return {fpc,pogo,e1,e2};
