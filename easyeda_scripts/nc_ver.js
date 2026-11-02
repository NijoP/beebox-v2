const out={};
for(const [name,doc] of [["main","3d113e2c2213fddc"],["patch","df98848100b24fd4"]]){
 await eda.dmt_EditorControl.openDocument(doc); await new Promise(r=>setTimeout(r,1500));
 const nl=await eda.sch_Netlist.getNetlist("Protel2");
 const lines=nl.split(/\r?\n/);const nets=[];let cur=null;
 for(const ln of lines){const t=ln.trim();if(t==="(")cur={pins:[]};else if(t===")"){if(cur)nets.push(cur);cur=null;}else if(cur){if(!cur.name)cur.name=t;else cur.pins.push(t.split(' ')[0]);}}
 const onNet=new Set(nets.flatMap(n=>n.pins));
 const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
 const openNoFlag=[];let ncCount=0;
 for(const c of cs){const d=c.getState_Designator();if(!d)continue;for(const p of await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(c.getState_PrimitiveId())){const k=d+"-"+p.getState_PinNumber();const nc=p.getState_NoConnected();if(nc)ncCount++;if(!onNet.has(k)&&!nc)openNoFlag.push(k);if(onNet.has(k)&&nc)openNoFlag.push("CONFLICT "+k);}}
 const m={};for(const n of nets)m[n.name]=n.pins.length;
 out[name]={components:cs.length,nets:nets.length,singles:nets.filter(n=>n.pins.length<2).map(n=>n.name),openWithoutNC:openNoFlag,ncFlags:ncCount,GND:m.GND,"+3V3":m["+3V3"]||m["+3V3_LOCAL"]};
}
return out;
