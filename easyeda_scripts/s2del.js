const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"wrong sheet"};
const box=(x,y)=>x>950&&x<2650&&y>-60&&y<760;
const ids=await eda.sch_PrimitiveComponent.getAllPrimitiveId();
const comps=await eda.sch_PrimitiveComponent.get(ids);
const dc=comps.filter(c=>c.getState_ComponentType&&c.getState_ComponentType()!=="sheet"&&box(c.getState_X(),c.getState_Y())).map(c=>c.getState_PrimitiveId());
if(dc.length) await eda.sch_PrimitiveComponent.delete(dc);
const wires=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());
const dw=[];for(const w of wires){let ln;try{ln=w.getState_Line();}catch(e){continue;}for(let i=0;i<ln.length;i+=2)if(box(ln[i],ln[i+1])){dw.push(w.getState_PrimitiveId());break;}}
if(dw.length) await eda.sch_PrimitiveWire.delete(dw);
const attrs=await eda.sch_PrimitiveAttribute.get(await eda.sch_PrimitiveAttribute.getAllPrimitiveId());
let da=0;for(const a of attrs){let x,y;try{x=a.getState_X();y=a.getState_Y();}catch(e){continue;}if(box(x,y)){try{await eda.sch_PrimitiveAttribute.delete([a.getState_PrimitiveId()]);da++;}catch(e){}}}
return {compsDeleted:dc.length,wiresDeleted:dw.length,attrsDeleted:da,left:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};
