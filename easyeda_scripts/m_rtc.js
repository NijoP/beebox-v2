const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main",DOC};
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
async function lbl(id,map){const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const g={};for(const p of pins){const r=""+p.getState_Rotation();(g[r]=g[r]||[]).push(p);}
 for(const r of Object.keys(g)){const arr=g[r];if(r==="180"||r==="0")arr.sort((a,b)=>b.getState_Y()-a.getState_Y());else arr.sort((a,b)=>a.getState_X()-b.getState_X());
  for(let k=0;k<arr.length;k++){const p=arr[k];const net=map[p.getState_PinName()];if(net==null)continue;const x=p.getState_X(),y=p.getState_Y(),R=+r,st=(k%2)*60;let ex=x,ey=y,lr=0;
   if(R===180)ex=x-(110+st);else if(R===0)ex=x+(110+st);else if(R===270){ey=y-(80+st);lr=90;}else{ey=y+(80+st);lr=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:lr,fontSize:5});}catch(e){}}}}
const rtc=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:"4c439d145251422d86fe12a14a648115"},2950,1550,"",0,false,true,true);
await lbl(rtc.getState_PrimitiveId(),{"VDD":"+3V3","VSS":"GND","SCL":"SYS_SCL","SDA":"SYS_SDA","INT#":"RTC_INT_N","VBACKUP":"VBAT_RTC","EVI":"GND"});
const bt=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid:"6c6537e92f8045d7b2cdb8b0ea44de27"},3400,1550,"",0,false,true,true);
await lbl(bt.getState_PrimitiveId(),{"1":"VBAT_RTC","2":"GND"});
await p2(C100n,2950,1400,"+3V3","GND");
await p2(C100n,3200,1400,"VBAT_RTC","GND");
await p2(R100k,3450,1400,"+3V3","RTC_INT_N");
const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const u1=cs.find(c=>c.getState_Designator()==="U1").getState_PrimitiveId();
const rr=await relabel(u1,"IO17","RTC_INT_N");
await eda.sch_PrimitiveText.create(2800,1680,"RTC  RV-3028-C7 (45 nA, ±1 ppm, I2C 0x52) + CR2032 backup",0,"#1569C7",null,14,true,false,false);
await eda.sch_PrimitiveText.create(2800,1300,"Firmware: enable backup switchover (BSM), keep trickle charge OFF (primary cell). Verify BT1 pin1 = '+' at PCB footprint.",0,"#1569C7",null,8,true,false,false);
return {rr,n:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};
