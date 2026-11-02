const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
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
const C10u50="7d63f541153841429f3d9086c7babcc4",C100n50="4724975b6c2a4e8dbf5d2a441323882f",L10u="438ae48248e246a9a68a6c56e8036c58",R137="a7dea258d14247b0968153eb1b4aa039",C22u="9583d53cf848486a9d523fcaf153a729",SS14="93c829a348834d679d8d8a059ea91636";
await p2(C10u50,650,2120,"VIN_PROT","GND");
await p2(C10u50,880,2120,"VIN_PROT","GND");
await p2(C100n50,1110,2120,"VIN_PROT","GND");
await p2(C100n,1340,2120,"CB_5V","SW_5V");
await p2(L10u,650,2000,"SW_5V","+5V0");
await p2(R100k,880,2000,"+5V0","FB_5V");
await p2(R137,1110,2000,"FB_5V","GND");
await p2(C22u,1340,2000,"+5V0","GND");
await p2(C22u,1570,2000,"+5V0","GND");
await p2(SS14,700,2620,"+5V0","VBUS");
await p2(R47k,950,2620,"VBUS","VBUS_DET");
await p2(R100k,1200,2620,"VBUS_DET","GND");
await p2(R100k,940,1120,"VBAT_SENSE","GND");
return {ok:1,n:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};
