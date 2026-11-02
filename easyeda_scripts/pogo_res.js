const out={};
for(const q of ["pogo pin 12P 2.2mm","pogo 2x6 2.2","spring loaded connector 12P 2.2mm","POGO PIN 2*6P","弹簧顶针 2.2 12P"]){const r=await eda.lib_Device.search(q);out[q]=(r||[]).slice(0,4).map(d=>[d.name,d.supplierId,d.footprintName]);}
return out;
