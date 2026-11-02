const f=async id=>(await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id)).map(p=>p.getState_PinName()+"@"+p.getState_X()+","+p.getState_Y()+" r"+p.getState_Rotation());
return {hdr:await f("152fd0aab6ac313e"),esd:await f("92959654b262ed12"),tps:await f("f6dc110747c6886e")};
