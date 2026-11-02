const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(!DOC||DOC.uuid!=="df98848100b24fd4") return {error:"wrong sheet",DOC};
const LIB="0819f05c4eef4c71ace90d822a990e87";
async function lbl(id,map){
 const pins=await eda.sch_PrimitiveComponent.getAllPinsByPrimitiveId(id);
 const g={}; for(const p of pins){const r=""+p.getState_Rotation();(g[r]=g[r]||[]).push(p);}
 for(const r of Object.keys(g)){const arr=g[r];
  if(r==="180"||r==="0")arr.sort((a,b)=>b.getState_Y()-a.getState_Y()); else arr.sort((a,b)=>a.getState_X()-b.getState_X());
  for(let k=0;k<arr.length;k++){const p=arr[k];const nm=p.getState_PinName();const net=map[nm];
   if(net==null||net==="")continue;const x=p.getState_X(),y=p.getState_Y(),R=+r,st=(k%2)*60;
   let ex=x,ey=y,lr=0;
   if(R===180)ex=x-(110+st); else if(R===0)ex=x+(110+st); else if(R===270){ey=y-(80+st);lr=90;} else {ey=y+(80+st);lr=90;}
   await eda.sch_PrimitiveWire.create([x,y,ex,ey],net);
   const L=await eda.sch_PrimitiveAttribute.createNetLabel(ex,ey,net);
   try{await eda.sch_PrimitiveAttribute.modify(L.getState_PrimitiveId(),{rotation:lr,fontSize:5});}catch(e){}}}
}
async function place(uuid,x,y){const c=await eda.sch_PrimitiveComponent.create({libraryUuid:LIB,uuid},x,y,"",0,false,true,true);return c.getState_PrimitiveId();}
const CH={"1":"GND","2":"+5V_CHAIN","3":"GND","4":"SENSOR_CLK","5":"SENSOR_SYNC","6":"GND","7":"SENSOR_DATA_ID","9":"PATCH_CLK","10":"PATCH_LATCH","11":"PATCH_OE_N","12":"AUX_ARM","13":"GND","14":"GND"};
const jin=await place("56d0813866664aa693617cc16753ea47",450,1500); await lbl(jin,{...CH,"8":"PATCH_DATA_IN"});
const jout=await place("56d0813866664aa693617cc16753ea47",450,1100); await lbl(jout,{...CH,"8":"PATCH_DATA_OUT"});
const reg=await place("b902ba68e4fd4b048ff83be09f873cbc",1050,1500); await lbl(reg,{"GND":"GND","VIN":"+5V_CHAIN","MODE":"GND","EN":"+5V_CHAIN","VSET":"VSET_R","STOP":"GND","SW":"SW_LOC","VOS":"+3V3_LOCAL"});
const sr=await place("f7296b70af1e425f9d78faa598566f18",1050,1100); await lbl(sr,{"VCC":"+3V3_LOCAL","GND":"GND","SER":"PATCH_DATA_IN","SRCLK":"PATCH_CLK","RCLK":"PATCH_LATCH","OE_n":"PATCH_OE_N","SRCLR_n":"+3V3_LOCAL","QH'":"QH_OUT","QH&apos;":"QH_OUT","QA":"PRI_PWR_EN","QB":"PRI_SIG_EN","QC":"ID_EN","QD":"AUX_A0","QE":"AUX_A1","QF":"AUX_SIG_EN","QG":"AUX3V3_EN","QH":"AUX5V_EN"});
const ls=await place("e30f49de94c341bea49e53f3016845f9",1650,1500); await lbl(ls,{"VIN":"+3V3_LOCAL","GND":"GND","ON":"PRI_PWR_EN","CT":"PRI_CT","QOD":"PRI_VDD","VOUT":"PRI_VDD"});
const sw=await place("4ab6cbe4c9f94cc58a3871090025db9c",1650,1100); await lbl(sw,{"VDD":"+3V3_LOCAL","GND":"GND","SEL1":"PRI_SIG_EN","SEL2":"PRI_SIG_EN","SEL3":"PRI_SIG_EN","SEL4":"ID_EN","D1":"SENSOR_CLK","S1":"CLK_LOC","D2":"SENSOR_SYNC","S2":"SYNC_LOC","D3":"SENSOR_DATA_ID","S3":"SD_LOC","D4":"SENSOR_DATA_ID","S4":"ID_NODE"});
const sht=await place("d2ae2b1001894b88873ae8d8ab95ec2d",2150,1500); await lbl(sht,{"SDA":"SYNC_LOC","SCL":"CLK_LOC","VDD":"PRI_VDD","VSS":"GND","5":"GND"});
const mic=await place("ea6d3dad996d464d91983d780e0fb112",2150,1150); await lbl(mic,{"WS":"SYNC_LOC","LR":"GND","GND":"GND","SCK":"CLK_LOC","VDD":"PRI_VDD","SD":"SD_LOC"});
return {jin,jout,reg,sr,ls,sw,sht,mic};
