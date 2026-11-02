await eda.dmt_EditorControl.openDocument("3d113e2c2213fddc"); await new Promise(r=>setTimeout(r,1500));
const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const want=["U2","U4","L2","R3","R4","R5","R6","R7","R8","R9","R10","R17","R18","R20","R21","C6","C7","C8","C9","C11","C12","C13","CN1","LED2","R26","U3"];
const o={};for(const c of cs){const d=c.getState_Designator();if(want.includes(d))o[d]={id:c.getState_PrimitiveId(),x:c.getState_X(),y:c.getState_Y(),mpn:c.getState_Name?c.getState_Name():""};}
return o;
