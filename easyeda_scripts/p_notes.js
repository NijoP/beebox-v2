const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"not patch"};
const ts=await eda.sch_PrimitiveText.get(await eda.sch_PrimitiveText.getAllPrimitiveId());
for(const t of ts){const c=t.getState_Content();if(c.startsWith("Q0 ")||c.startsWith("Every patch")) await eda.sch_PrimitiveText.delete([t.getState_PrimitiveId()]);}
async function T(x,y,s,sz){await eda.sch_PrimitiveText.create(x,y,s,0,"#1569C7",null,sz||8,true,false,false);}
await T(150,140,"Q0 PRI_PWR(mic rail) | Q1 PRI_SIG | Q2 ID | Q3 MOD_PWR(SHT45+AUX rail) | Q4 MOD_SIG | Q5 PRES_T | Q6 PRES_M | Q7 spare");
await T(150,110,"Switch enables are hardware-gated: MIC_SEL = Q1 AND PRI_VDD (U10), MOD_SEL = Q4 AND MOD_3V3 (U11)  -> no back-powering");
await T(1400,1200,"ASSEMBLY VARIANTS (same PCB):  A = SHT45 only  |  B = mic only  |  C = both  |  D = neither");
await T(1400,1175,"Group T (fit with SHT45): U5 SHT45, C10, R14 68k(PRES_T)    Group M (fit with mic): U6 ICS-43434, C11, R15 68k(PRES_M)");
await T(1400,1150,"ID: Q2 -> 2.70V = patch present;  Q5 -> 1.89V SHT45 fitted / 0V;  Q6 -> 1.89V mic fitted / 0V");
return {ok:1};
