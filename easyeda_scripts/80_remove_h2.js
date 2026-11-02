const box=(x,y)=>x>2680&&x<3230&&y>1350&&y<1760;
const wids=await eda.sch_PrimitiveWire.getAllPrimitiveId();
const wires=await eda.sch_PrimitiveWire.get(wids);
const delW=[];
for(const w of wires){let ln;try{ln=w.getState_Line();}catch(e){continue;} let hit=false;
 for(let i=0;i<ln.length;i+=2){if(box(ln[i],ln[i+1]))hit=true;} if(hit)delW.push(w.getState_PrimitiveId());}
if(delW.length)await eda.sch_PrimitiveWire.delete(delW);
const aids=await eda.sch_PrimitiveAttribute.getAllPrimitiveId();
const attrs=await eda.sch_PrimitiveAttribute.get(aids);
const delA=[];
for(const a of attrs){let x,y;try{x=a.getState_X();y=a.getState_Y();}catch(e){continue;} if(box(x,y))delA.push(a.getState_PrimitiveId());}
for(const id of delA){try{await eda.sch_PrimitiveAttribute.delete([id]);}catch(e){}}
try{await eda.sch_PrimitiveComponent.delete(["9d4f168890117333"]);}catch(e){}
return {wiresDeleted:delW.length,attrsDeleted:delA.length,components:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};
