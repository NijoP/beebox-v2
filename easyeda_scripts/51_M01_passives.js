const LIB = "0819f05c4eef4c71ace90d822a990e87";
const R22k="ab7b2f8b72664e3fb9f5bed85b07623d",R47k="67bcedcf1a474734ac65b62dc52125d6",R10k="ffb8c4ed7d7244bc918e37313d4ca373",R100k="a8fe6fdf29924f6c9dcbfa5a84a02fec",R51k="3614094eae9e47dbbbde1f877b2c30ef",R562k="311efdff25ca4ed086c602bd49bb7bb2",R750k="b899ecf891d4423c815cfc65f6febb4f",R1M="72d71792aca642bfa6283f6c1d1961c3",R330k="01c5497fe62948148db1754795f652db",R768k="8866aaee3e9549c88c7f31286c8f6648";
const C1u="115c3169541a48b3ba9c88523dfbcdee",C10u="b170840a71594691a5c5caad0ed65e15",C100n="2eaf9ba597d94b93917d2b625402739a";
const L15="64e29f11414b4167aed61e2847fee31d",L10="6425493c50844b98aee9c9b07d0e08c8";
const PLAN=[
 [R22k,"BQ_ISET","GND"],[R22k,"BQ_ILIM","GND"],[R47k,"BQ_TMR","GND"],[R10k,"BQ_TS","GND"],
 [R100k,"+3V3","CHG_EN1"],[R100k,"CHG_EN2","GND"],[R100k,"+3V3","CHG_STAT_N"],[R100k,"+3V3","CHG_PGOOD_N"],
 [R100k,"+3V3","REG_PG"],[R100k,"+3V3","CHAIN_FAULT_N"],[R51k,"CC1","GND"],[R51k,"CC2","GND"],
 [R562k,"+3V3","FB_3V3"],[R100k,"FB_3V3","GND"],[R750k,"+5V0","FB_5V0"],[R100k,"FB_5V0","GND"],
 [R768k,"TPS2553_ILIM","GND"],[R1M,"VBAT","VBAT_SENSE"],[R330k,"VBAT_SENSE","GND"],
 [C1u,"VBUS","GND"],[C10u,"VSYS","GND"],[C10u,"VBAT","GND"],[C10u,"VSYS","GND"],[C10u,"+3V3","GND"],
 [C10u,"VSYS","GND"],[C10u,"+5V0","GND"],[C1u,"+5V0","GND"],[C10u,"+5V_CHAIN","GND"],[C100n,"VBAT_SENSE","GND"],
 [L15,"L_3V3A","L_3V3B"],[L10,"VSYS","L_5V0"]
];
const cols=6, stepX=300, stepY=150, x0=200, y0=2350;
let i=0;
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
