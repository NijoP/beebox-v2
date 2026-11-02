const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const j=cs.find(c=>c.getState_Designator()==="DC1");
const pins=(await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(j.getState_PrimitiveId())).map(p=>({n:p.getState_PinNumber(),x:p.getState_X(),y:p.getState_Y(),r:p.getState_Rotation()}));
const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());
const near=ws.filter(w=>{const l=w.getState_Line();return l.some((v,i)=>i%2===0&&Math.abs(v-900)<300&&Math.abs(l[i+1]-3250)<120);}).map(w=>({l:w.getState_Line(),net:w.getState_Net?w.getState_Net():""}));
return {id:j.getState_PrimitiveId(),pins,near};
