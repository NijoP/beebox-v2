const LIB = "0819f05c4eef4c71ace90d822a990e87";
const C1u="115c3169541a48b3ba9c88523dfbcdee",C10u="b170840a71594691a5c5caad0ed65e15",C100n="2eaf9ba597d94b93917d2b625402739a";
const L15="64e29f11414b4167aed61e2847fee31d",L10="6425493c50844b98aee9c9b07d0e08c8";
const PLAN=[
 [C10u,"+3V3","GND"],[C10u,"VSYS","GND"],[C10u,"+5V0","GND"],[C1u,"+5V0","GND"],
 [C10u,"+5V_CHAIN","GND"],[C100n,"VBAT_SENSE","GND"],[L15,"L_3V3A","L_3V3B"],[L10,"VSYS","L_5V0"]
];
const cols=6, stepX=300, stepY=150, x0=200, y0=2350;
let i=23;
for (const [uuid,netA,netB] of PLAN) {
  const col=i%cols, row=Math.floor(i/cols);
  const x=x0+col*stepX, y=y0+row*stepY;
  const c = await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid}, x, y, "", 0, false, true, true);
  const pins = await c.getAllPins();
  const pi = pins.map(p=>({x:p.getState_X(),y:p.getState_Y()}));
  const xs=[...new Set(pi.map(p=>p.x))].sort((a,b)=>a-b);
  for (const p of pi) {
    const isLeft = p.x===xs[0];
    const net = isLeft?netA:netB;
    const ex = isLeft ? p.x-120 : p.x+55;
    await eda.sch_PrimitiveWire.create([p.x,p.y,ex,p.y], net);
    const lb = await eda.sch_PrimitiveAttribute.createNetLabel(ex,p.y,net);
    try{ await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5}); }catch(e){}
  }
  i++;
}
return { placed: PLAN.length };
