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
const p3=await place("921d0fc4bb4347dca0bd09d3e42f87c6",1150,620); await lbl(p3,{"IN":"+3V3_LOCAL","GND":"GND","EN":"AUX3V3_EN","FAULT#":"AUX3V3_FLT_N","ILIM":"AUX3V3_ILIM","OUT":"AUX_3V3"});
const p5=await place("921d0fc4bb4347dca0bd09d3e42f87c6",1150,440); await lbl(p5,{"IN":"+5V_CHAIN","GND":"GND","EN":"AUX5V_EN","FAULT#":"AUX5V_FLT_N","ILIM":"AUX5V_ILIM","OUT":"AUX_5V"});
const Y=[400,290,180,70];
for(let n=1;n<=4;n++){
 const j=await place("71bdd799d1af4500b01fdf95cb8f7869",2300,Y[n-1]); await lbl(j,{"1":"AUX_3V3","2":"GND","3":"AUX"+n+"_SDA","4":"AUX"+n+"_SCL","5":"AUX_5V"});
 const e=await place("71bd5434893a460c8be823067c2f4e47",1750,Y[n-1]); await lbl(e,{"D1+":"AUX"+n+"_SDA","D1-":"AUX"+n+"_SCL","D2+":"AUX_3V3","D2-":"AUX_5V","GND":"GND"});
}
return {ok:1};
