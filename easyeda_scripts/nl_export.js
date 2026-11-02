const d=await eda.dmt_SelectControl.getCurrentDocumentInfo();
const nl=await eda.sch_Netlist.getNetlist("Protel2");
const cs=await eda.sch_PrimitiveComponent.get(await eda.sch_PrimitiveComponent.getAllPrimitiveId());
const parts=cs.map(c=>({ref:c.getState_Designator(),x:c.getState_X(),y:c.getState_Y(),dev:(c.getState_Component?c.getState_Component():null)}));
return {doc:d.uuid,nl,parts};
