const DOC=await eda.dmt_SelectControl.getCurrentDocumentInfo(); if(DOC.uuid!=="3d113e2c2213fddc") return {error:"not main"};
const t=await eda.sch_PrimitiveText.getAllPrimitiveId(); if(t.length) await eda.sch_PrimitiveText.delete(t);
async function T(x,y,s,sz){await eda.sch_PrimitiveText.create(x,y,s,0,"#1569C7",null,sz||14,true,false,false);}
await T(1650,2980,"MCU  ESP32-S3 (native USB programming)");
await T(600,3060,"BATTERY IN  6S BMS P+/P-  ->  PTC  ->  SS34  ->  TVS");
await T(1080,2490,"5V BUCK  LMR51420  (15-25.2V -> 4.98V, 2A)");
await T(120,2560,"USB-C  (data + 5V backup via SS14)");
await T(120,1900,"+3V3 TPS63802   |   +5V_CHAIN TPS2553 (340 mA, latch-off, default OFF)");
await T(120,1600,"POWER PASSIVES (FB dividers, pull-ups, decoupling, VBAT sense)");
await T(1780,1960,"POWER BUTTON (soft-latch, diode-OR)");
await T(2800,2860,"CHAIN CONTROL");
await T(3300,2860,"STATUS LEDs");
await T(3650,2720,"CHAIN CONNECTORS  FPC + POGO (same 12 nets)");
await T(1600,1180,"DISPLAY HEADER + ONBOARD SHT45 / ICS-43434");
await T(600,3010,"Charge the 6S pack with an external 25.2V CC/CV charger at the BMS (no on-board charger)",8);
return {ok:1};
