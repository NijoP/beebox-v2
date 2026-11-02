const PLAN={
 "3d113e2c2213fddc":{single:["U1-37","U1-36","U1-35","U1-31","U1-30","U1-28","U1-26","U1-16","U1-15","U1-11","U1-8","USB1-B8","USB1-A8"],open:["D1-1","D1-2","D2-6","D2-7","D2-9","D2-10","D3-6","D3-7","D3-9","D3-10","U9-1"]},
 "df98848100b24fd4":{single:[],open:["U2-7","D1-5","D1-6","D1-7","D1-9","D1-10"]}};
const res={};
for(const doc of Object.keys(PLAN)){
 await eda.dmt_EditorControl.openDocument(doc); await new Promise(r=>setTimeout(r,1500));
 const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
 const byRef={};for(const c of cs){const d=c.getState_Designator();if(d)byRef[d]=c.getState_PrimitiveId();}
 const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());
 let wdel=0,nc=0;const errs=[];
 for(const key of [...PLAN[doc].single,...PLAN[doc].open]){
  const [ref,num]=[key.slice(0,key.lastIndexOf("-")),key.slice(key.lastIndexOf("-")+1)];
  const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(byRef[ref]);const p=pins.find(q=>q.getState_PinNumber()===num);
  if(!p){errs.push("nopin "+key);continue;}
  if(PLAN[doc].single.includes(key)){const x=p.getState_X(),y=p.getState_Y();
   const d=ws.filter(w=>{const l=w.getState_Line();return (l[0]===x&&l[1]===y)||(l[l.length-2]===x&&l[l.length-1]===y);}).map(w=>w.getState_PrimitiveId());
   if(d.length){await eda.sch_PrimitiveWire.delete(d);wdel+=d.length;}}
  try{p.setState_NoConnected(true);await p.done();nc++;}catch(e){errs.push(key+" "+e);}
 }
 res[doc]={wiresDeleted:wdel,ncSet:nc,errs};
}
return res;
