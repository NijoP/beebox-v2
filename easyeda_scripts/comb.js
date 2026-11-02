await eda.dmt_EditorControl.openDocument("3d113e2c2213fddc"); await new Promise(r=>setTimeout(r,1500));
const nl=await eda.sch_Netlist.getNetlist("Protel2");
const all=await eda.sch_PrimitiveComponent.getAllPrimitiveId(undefined,true);
const cs=await eda.sch_PrimitiveComponent.get(all);
return {nl,refs:cs.map(c=>c.getState_Designator()).filter(Boolean)};
