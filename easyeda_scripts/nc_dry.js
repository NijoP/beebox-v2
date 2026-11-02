const out={};
for(const [name,doc] of [["main","3d113e2c2213fddc"],["patch","df98848100b24fd4"]]){
 await eda.dmt_EditorControl.openDocument(doc); await new Promise(r=>setTimeout(r,1500));
 const nl=await eda.sch_Netlist.getNetlist("Protel2");
 const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
 for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t.split(' ')[0]);}}
 const onNet=new Set(nets.flatMap(n=>n.pins));
 const singles=nets.filter(n=>n.pins.length===1).map(n=>n.name+"="+n.pins[0]);
 const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
 const open=[];
 for(const c of cs){const d=c.getState_Designator();if(!d)continue;const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(c.getState_PrimitiveId());
  for(const p of ps){const k=d+"-"+p.getState_PinNumber();if(!onNet.has(k))open.push(k+"("+p.getState_PinName()+")"+(p.getState_NoConnected()?"[NC]":""));}}
 out[name]={singles,open};
}
return out;
