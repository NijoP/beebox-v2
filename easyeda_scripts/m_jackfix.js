const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
async function pinXY(id,key){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const p=ps.find(q=>q.getState_PinNumber()===key||q.getState_PinName()===key);return p?[p.getState_X(),p.getState_Y()]:null;}
async function wiresAt(x,y){const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());const r=[];for(const w of ws){const l=w.getState_Line();if((l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y))r.push(w);}return r;}
async function relabel(id,key,net){const xy=await pinXY(id,key);if(!xy)return "nopin "+key;const ws=await wiresAt(xy[0],xy[1]);if(!ws.length)return "nowire";const l=ws[0].getState_Line();const end=(l[0]===xy[0]&&l[1]===xy[1])?[l[l.length-2],l[l.length-1]]:[l[0],l[1]];
 await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));await eda.sch_PrimitiveWire.create([xy[0],xy[1],end[0],end[1]],net);
 const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(xy[0]===end[0])?90:0,fontSize:5});}catch(e){}return "ok";}
async function removePart(id){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){const ws=await wiresAt(p.getState_X(),p.getState_Y());if(ws.length)await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));}await eda.sch_PrimitiveComponent.delete([id]);}
async function partAt(x,y){const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());const c=cs.find(c=>c.getState_X()===x&&c.getState_Y()===y);return c?c.getState_PrimitiveId():null;}
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
const f2=await partAt(1150,3250);
const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(f2);const pts=new Set(ps.map(p=>p.getState_X()+","+p.getState_Y()));
const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());
const dw=ws.filter(w=>{const l=w.getState_Line();return pts.has(l[0]+","+l[1])||pts.has(l[l.length-2]+","+l[l.length-1]);}).map(w=>w.getState_PrimitiveId());
await eda.sch_PrimitiveWire.delete(dw); await eda.sch_PrimitiveComponent.delete([f2]);
// re-create jack pin-3 GND stub cleanly (pin3 at 920,3250)
const w3=(await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId())).filter(w=>{const l=w.getState_Line();return l[0]===920&&l[1]===3250;});
let fixed3="none"; if(w3.length){await eda.sch_PrimitiveWire.delete(w3.map(w=>w.getState_PrimitiveId())); await eda.sch_PrimitiveWire.create([920,3250,1090,3250],"GND"); const L=await eda.sch_PrimitiveAttribute.createNetLabel(1090,3250,"GND"); try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{fontSize:5});}catch(e){} fixed3="ok";}
await p2("6c49915692fa40ac8af2f201facd8b0a",1400,3420,"CHG_IN","BAT_IN");
return {wiresDel:dw.length,fixed3};
