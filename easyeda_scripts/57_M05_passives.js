const LIB = "0819f05c4eef4c71ace90d822a990e87";
const C100n="2eaf9ba597d94b93917d2b625402739a",R22k="ab7b2f8b72664e3fb9f5bed85b07623d",R1k="17aa4e57569d4d22adc28d945ba5559d";
const LEDr="015cbd77079d4b0ea5c81b79fefdcea3",LEDg="e8b07cfe9d254e27bcc3c1116be11e2b",LEDb="1a0921f6f61a426b8b7f0aa3c5392e45";
const PLAN=[
 [C100n,"+3V3","GND"],[C100n,"+3V3","GND"],
 [R22k,"SYS_SDA","+3V3"],[R22k,"SYS_SCL","+3V3"],
 [R1k,"LED_R","LEDR_A"],[R1k,"LED_G","LEDG_A"],[R1k,"LED_B","LEDB_A"],
 [LEDr,"GND","LEDR_A"],[LEDg,"GND","LEDG_A"],[LEDb,"GND","LEDB_A"]
];
const cols=2, stepX=320, stepY=140, x0=2850, y0=300;
let i=0;
for(const [uuid,netA,netB] of PLAN){
  const col=i%cols,row=Math.floor(i/cols);
  const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid}, x0+col*stepX, y0+row*stepY, "", 0, false, true, true);
  const pins=await c.getAllPins(); const pi=pins.map(p=>({x:p.getState_X(),y:p.getState_Y()}));
  const xs=[...new Set(pi.map(p=>p.x))].sort((a,b)=>a-b);
  for(const p of pi){const isLeft=p.x===xs[0];const net=isLeft?netA:netB;const ex=isLeft?p.x-120:p.x+55;
   await eda.sch_PrimitiveWire.create([p.x,p.y,ex,p.y],net);
   const lb=await eda.sch_PrimitiveAttribute.createNetLabel(ex,p.y,net);
   try{await eda.sch_PrimitiveAttribute.modify(lb.getState_PrimitiveId(),{fontSize:5});}catch(e){}}
  i++;
}
return { placed: PLAN.length };
