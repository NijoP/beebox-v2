const comps = (await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length;
const wires = (await eda.sch_PrimitiveWire.getAllPrimitiveId()).length;
const attrs = (await eda.sch_PrimitiveAttribute.getAllPrimitiveId()).length;
const names = await eda.sch_Net.getAllNetsName();
const nl = await eda.sch_Netlist.getNetlist("Protel2");
const netCount = (nl.match(/\(/g)||[]).length;
return { comps, wires, attrs, netBlocks: netCount, sampleNets: (names||[]).slice(0,10) };
