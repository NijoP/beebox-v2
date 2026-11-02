async function T(x,y,s){try{await eda.sch_PrimitiveText.create(x,y,s,0,"#1569C7",null,16,true,false,false);}catch(e){return "ERR:"+e;}}
await T(1650,2980,"MCU - ESP32-S3 (USB prog)");
await T(400,3080,"POWER / CHARGER");
await T(1780,1800,"POWER BUTTON");
await T(2800,2860,"CHAIN CONTROL");
await T(3300,2860,"STATUS LEDs");
await T(3650,2720,"CHAIN CONNECTORS (FPC + POGO)");
await T(1600,1180,"DISPLAY + ONBOARD SENSORS");
return {ok:1};
