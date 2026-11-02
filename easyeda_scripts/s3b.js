const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(!DOC||DOC.uuid!=="df98848100b24fd4") return {error:"wrong sheet",DOC};
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
const r={};
r.pw=await place("921d0fc4bb4347dca0bd09d3e42f87c6",600,450); await lbl(r.pw,{"IN":"+3V3_LOCAL","GND":"GND","EN":"MOD_PWR_EN","FAULT#":"MOD_FLT_N","ILIM":"MOD_ILIM","OUT":"MOD_3V3"});
r.msw=await place("4ab6cbe4c9f94cc58a3871090025db9c",1150,450); await lbl(r.msw,{"VDD":"+3V3_LOCAL","GND":"GND","SEL1":"MOD_SIG_EN","SEL2":"MOD_SIG_EN","SEL3":"GND","SEL4":"GND","D1":"SENSOR_CLK","S1":"MOD_SCL","D2":"SENSOR_SYNC","S2":"MOD_SDA","D3":"GND","S3":"GND","D4":"GND","S4":"GND"});
r.esd=await place("71bd5434893a460c8be823067c2f4e47",1650,450); await lbl(r.esd,{"D1+":"MOD_SDA","D1-":"MOD_SCL","D2+":"MOD_3V3","GND":"GND"});
r.jm=await place("f61f530515f040b2b0bc6d26df5b0e4f",2150,450); await lbl(r.jm,{"1":"MOD_3V3","2":"GND","3":"MOD_SDA","4":"MOD_SCL"});
return r;
