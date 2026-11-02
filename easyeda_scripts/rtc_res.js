const out={};
for(const q of ["RV-3028-C7","RV-3028-C7 32.768kHz 1ppm-TA-QC","CR2032-BS-6-1","BS-08-B2AA003"]){const r=await eda.lib_Device.search(q);out[q]=(r||[]).slice(0,2).map(d=>[d.name,d.supplierId,d.footprintName,d.uuid,(d.otherProperty||{}).Datasheet]);}
return out;
