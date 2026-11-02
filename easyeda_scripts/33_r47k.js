const r=await eda.lib_Device.search("0402WGF4702TCE");
return (r||[]).slice(0,1).map(d=>({name:d.name,lcsc:d.supplierId,fp:d.footprintName,dev:d.uuid}));
