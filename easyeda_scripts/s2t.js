const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="df98848100b24fd4") return {error:"wrong sheet"};
async function T(x,y,s,sz){await eda.sch_PrimitiveText.create(x,y,s,0,"#1569C7",null,sz||14,true,false,false);}
await T(200,1640,"CHAIN IN / OUT (FPC, pin 8 cascade)");
await T(800,1640,"LOCAL POWER  +5V_CHAIN -> +3V3_LOCAL");
await T(800,1230,"SHIFT REGISTER 74LV595 + FAIL-SAFE");
await T(1450,1640,"PRIMARY SENSOR (T/RH | MIC) + ID");
await T(200,560,"AUX I2C PORT MUX");
await T(1000,760,"AUX POWER (3V3 / 5V, current-limited)");
await T(1650,500,"AUX MODULE PORTS x4");
await T(1400,1000,"VARIANT T/RH: fit U5 SHT45 + R12 22k | DNI U6, R13",8);
await T(1400,970,"VARIANT MIC : fit U6 ICS-43434 + R13 68k | DNI U5, R12",8);
await T(1650,-30,"J_AUX: 1 3V3 | 2 GND | 3 SDA | 4 SCL | 5 5V (power only, 3.3V logic)",8);
return {ok:1};
