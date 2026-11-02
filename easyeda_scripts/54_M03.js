const LIB = "0819f05c4eef4c71ace90d822a990e87";
const R33="1f1e772b30d54f119255cd0da7dd806e",R1k="17aa4e57569d4d22adc28d945ba5559d",R100k="a8fe6fdf29924f6c9dcbfa5a84a02fec",R22k="ab7b2f8b72664e3fb9f5bed85b07623d";
const PLAN=[
 [R33,"SENSOR_CLK","SENSOR_CLK_C"],[R33,"SENSOR_SYNC","SENSOR_SYNC_C"],
 [R33,"PATCH_SHIFT_DATA","PATCH_DATA_C"],[R33,"PATCH_SHIFT_CLK","PATCH_CLK_C"],
 [R33,"PATCH_LATCH","PATCH_LATCH_C"],[R1k,"PATCH_OE_N","PATCH_OE_N_C"],
 [R100k,"SENSOR_DATA_ID","GND"],[R100k,"AUX_ARM","GND"],
 [R22k,"SENSOR_CLK","SENSOR_PU_EN"],[R22k,"SENSOR_SYNC","SENSOR_PU_EN"]
];
const cols=5, stepX=300, stepY=150, x0=1850, y0=1700;
let i=0;
for (const [uuid,netA,netB] of PLAN) {
  const col=i%cols,row=Math.floor(i/cols);
  const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid}, x0+col*stepX, y0+row*stepY, "", 0, false, true, true);
  const pins=await c.getAllPins(); const pi=pins.map(p=>({x:p.getState_X(),y:p.getState_Y()}));
  const xs=[...new Set(pi.map(p=>p.x))].sort((a,b)=>a-b);
  for(const p of pi){const isLeft=p.x===xs[0];const net=isLeft?netA:netB;const ex=isLeft?p.x-130:p.x+55;
   await eda.sch_PrimitiveWire.create([p.x,p.y,ex,p.y],net);
   const lb=await eda.sch_PrimitiveAttribute.createNetLabel(ex,p.y,net);
   try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5});}catch(e){}}
  i++;
}
return { placed: PLAN.length };
