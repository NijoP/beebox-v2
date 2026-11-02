const out={};
for(const q of ["SMD1812P200TF/30","SMD2920P200TF/30","1812L200/33MR","SMD1812P150TF/33","mSMD200-33V","BSMD1812-200-33V"]){const r=await eda.lib_Device.search(q);out[q]=(r||[]).slice(0,2).map(d=>[d.name,d.supplierId,d.footprintName,d.uuid]);}
return out;
