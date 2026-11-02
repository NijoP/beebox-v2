await eda.dmt_EditorControl.openDocument("df98848100b24fd4"); await new Promise(r=>setTimeout(r,1500)); const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"not patch",DOC};
async function pinXY(id,key){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const p=ps.find(q=>q.getState_PinNumber()===key||q.getState_PinName()===key);return p?[p.getState_X(),p.getState_Y()]:null;}
async function wiresAt(x,y){const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());const r=[];for(const w of ws){const l=w.getState_Line();if((l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y))r.push(w);}return r;}
async function relabel(id,key,net){const xy=await pinXY(id,key);if(!xy)return "nopin "+key;const ws=await wiresAt(xy[0],xy[1]);if(!ws.length)return "nowire";const l=ws[0].getState_Line();const end=(l[0]===xy[0]&&l[1]===xy[1])?[l[l.length-2],l[l.length-1]]:[l[0],l[1]];
 await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));await eda.sch_PrimitiveWire.create([xy[0],xy[1],end[0],end[1]],net);
 const L=await eda.sch_PrimitiveAttribute.createNetLabel(end[0],end[1],net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:(xy[0]===end[0])?90:0,fontSize:5});}catch(e){}return "ok";}
async function removePart(id){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps){const ws=await wiresAt(p.getState_X(),p.getState_Y());if(ws.length)await eda.sch_PrimitiveWire.delete(ws.map(w=>w.getState_PrimitiveId()));}await eda.sch_PrimitiveComponent.delete([id]);}
async function partAt(x,y){const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());const c=cs.find(c=>c.getState_X()===x&&c.getState_Y()===y);return c?c.getState_PrimitiveId():null;}
async function lbl(id,map){const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);const g={};for(const p of pins){const r=""+p.getState_Rotation();(g[r]=g[r]||[]).push(p);}
 for(const r of Object.keys(g)){const arr=g[r];if(r==="180"||r==="0")arr.sort((a,b)=>b.getState_Y()-a.getState_Y());else arr.sort((a,b)=>a.getState_X()-b.getState_X());
  for(let k=0;k<arr.length;k++){const p=arr[k];const net=map[p.getState_PinName()];if(net==null)continue;const x=p.getState_X(),y=p.getState_Y(),R=+r,st=(k%2)*60;let ex=x,ey=y,lr=0;
   if(R===180)ex=x-(110+st);else if(R===0)ex=x+(110+st);else if(R===270){ey=y-(80+st);lr=90;}else{ey=y+(80+st);lr=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:lr,fontSize:5});}catch(e){}}}}
const U4="9cfb1449380b5b89",U8="c6d9aadbd827d228",SR="71ca83131e5ab956";const r={};
for(const s of ["SEL1","SEL2","SEL3"]) r["u4"+s]=await relabel(U4,s,"MIC_SEL");
r.u8s1=await relabel(U8,"SEL1","MOD_SEL"); r.u8s2=await relabel(U8,"SEL2","MOD_SEL");
r.u8s3=await relabel(U8,"SEL3","PRES_T_EN"); r.u8d3=await relabel(U8,"D3","SENSOR_DATA_ID"); r.u8S3=await relabel(U8,"S3","PRES_T");
r.u8s4=await relabel(U8,"SEL4","PRES_M_EN"); r.u8d4=await relabel(U8,"D4","SENSOR_DATA_ID"); r.u8S4=await relabel(U8,"S4","PRES_M");
await lbl(SR,{"QF":"PRES_T_EN","QG":"PRES_M_EN"});
return r;
