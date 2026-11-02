const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
const ids=["7555c30f95bd7ab5","81440531f433bffc","3e7acfe9a903fab6","821548f6745e71e4","d91193d7009609e3","daf03287f4763494","16b54452174e063f","6a019f69ee99e903","316fb0eccd1e51d0","8f02c13d49aa43a9","7650a52b3dd83c90","cb9e1d19ac4e663b","e4331fb7867898d7","cbf1d3f5c90a2e82","a52966bdb6f4cc51","3f6d9bbd44738ba7","27204075081c65c5","ffb9625af17f94be","8ed9ea6c6d90b712","5ed594a8f51a8704"];
const pts=new Set();
for(const id of ids){const ps=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);for(const p of ps)pts.add(p.getState_X()+","+p.getState_Y());}
const ws=await eda.sch_PrimitiveWire.get(await eda.sch_PrimitiveWire.getAllPrimitiveId());
const dw=ws.filter(w=>{const l=w.getState_Line();return pts.has(l[0]+","+l[1])||pts.has(l[l.length-2]+","+l[l.length-1]);}).map(w=>w.getState_PrimitiveId());
await eda.sch_PrimitiveWire.delete(dw);
await eda.sch_PrimitiveComponent.delete(ids);
return {wiresDeleted:dw.length,partsDeleted:ids.length,left:(await eda.sch_PrimitiveComponent.getAllPrimitiveId()).length};
