const want=["LMR51420XDDCR","B2B-XH-A(LF)(SN)","SMD1812P110TF/33","SS34","SMBJ26A","CL31A106KBHNNNE","0402WGF1372TCE","SWPA6045S100MT","CL05B104KB54PNC"];
const out={};
for(const q of want){const r=await eda.lib_Device.search(q);const d=(r||[]).find(x=>x.name===q)||(r||[])[0];out[q]=d?[d.name,d.supplierId,d.footprintName,d.uuid]:"none";}
return out;
