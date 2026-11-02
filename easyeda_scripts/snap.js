const out={};
for(const [name,doc] of [["main","3d113e2c2213fddc"],["patch","df98848100b24fd4"]]){
 await eda.dmt_EditorControl.openDocument(doc); await new Promise(r=>setTimeout(r,1500));
 const nl=await eda.sch_Netlist.getNetlist("Protel2");
 const lines=nl.split(/\r?\n/);let cur=null;const pinNet={};
 for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)for(const p of cur.pins)pinNet[p]=cur.name;cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t.split(' ')[0]);}}
 const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
 const parts=[];
 for(const c of cs){const ref=c.getState_Designator();if(!ref)continue;const comp=c.getState_Component();
  const pins=(await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(c.getState_PrimitiveId())).map(p=>({num:p.getState_PinNumber(),name:p.getState_PinName(),net:pinNet[ref+"-"+p.getState_PinNumber()]||null,nc:!!p.getState_NoConnected()}));
  parts.push({ref,dev:comp,x:c.getState_X(),y:c.getState_Y(),pins});}
 out[name]={nl,parts};
}
return out;
