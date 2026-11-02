const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(!DOC||DOC.uuid!=="df98848100b24fd4") return {error:"wrong sheet",DOC};
const LIB="0819f05c4eef4c71ace90d822a990e87";
async function p2(uuid,x,y,na,nb){
 const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid},x,y,"",0,false,true,true);
 const pins=await c.getAllPins();const pi=pins.map(p=>({x:p.getState_X(),y:p.getState_Y()}));
 const xs=[...new Set(pi.map(p=>p.x))].sort((a,b)=>a-b);
 for(const p of pi){const isL=p.x===xs[0];const net=isL?na:nb;const ex=isL?p.x-100:p.x+55;
  await eda.sch_PrimitiveWire.create([p.x,p.y,ex,p.y],net);
  const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,p.y,net);
  try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{fontSize:5});}catch(e){}}
}
const R22k="ab7b2f8b72664e3fb9f5bed85b07623d",R47k="67bcedcf1a474734ac65b62dc52125d6",R10k="ffb8c4ed7d7244bc918e37313d4ca373",R100k="a8fe6fdf29924f6c9dcbfa5a84a02fec",R51k="3614094eae9e47dbbbde1f877b2c30ef",R562k="311efdff25ca4ed086c602bd49bb7bb2",R750k="b899ecf891d4423c815cfc65f6febb4f",R1M="72d71792aca642bfa6283f6c1d1961c3",R330k="01c5497fe62948148db1754795f652db",R768k="8866aaee3e9549c88c7f31286c8f6648",R1k="17aa4e57569d4d22adc28d945ba5559d";
const C1u="115c3169541a48b3ba9c88523dfbcdee",C10u="b170840a71594691a5c5caad0ed65e15",C100n="2eaf9ba597d94b93917d2b625402739a";
const L15="64e29f11414b4167aed61e2847fee31d",L10="6425493c50844b98aee9c9b07d0e08c8";
const SW="38630614a58f4650bd5eefe9a5f79d9a",LEDr="015cbd77079d4b0ea5c81b79fefdcea3",LEDg="e8b07cfe9d254e27bcc3c1116be11e2b",LEDb="1a0921f6f61a426b8b7f0aa3c5392e45";
const R931="59ffbf90b9ad49aaa3a5e1cedea45c77";
const P=[[C100n,"+3V3_LOCAL","GND"],[R931,"AUX3V3_ILIM","GND"],[R931,"AUX5V_ILIM","GND"],[R100k,"+3V3_LOCAL","AUX3V3_FLT_N"],[R100k,"+3V3_LOCAL","AUX5V_FLT_N"],[C1u,"+3V3_LOCAL","GND"],[C1u,"+5V_CHAIN","GND"],[C10u,"AUX_3V3","GND"],[C10u,"AUX_5V","GND"]];
let i=0;for(const[u,a,b]of P){const c=i%3,r=Math.floor(i/3);await p2(u,1500+c*270,680-r*100,a,b);i++;}
return {ok:P.length};
