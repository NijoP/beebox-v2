const pins = await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId("30ec681e6ed40e72");
const out = pins.map(p => ({ num: p.getState_PinNumber(), name: p.getState_PinName(), x: p.getState_X(), y: p.getState_Y(), rot: p.getState_Rotation() }));
out.sort((a,b)=>(parseInt(a.num)||0)-(parseInt(b.num)||0));
return out;
