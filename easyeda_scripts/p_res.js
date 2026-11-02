const want=["SN74LV595APWR","TPS62840DLCR","TMUX1511PWR","TMUX1109PWR","TPS2552DBVR-1","B5B-PH-K-S(LF)(SN)","SWPA3015S2R2NT","CL10A475KO8NNNC","0402WGF2673TCE","0402WGF2202TCE","0402WGF6802TCE","0402WGF9312TCE","0402B102K500NT"];
const out={};
for(const q of want){try{const r=await eda.lib_Device.search(q);out[q]=(r||[]).slice(0,1).map(d=>[d.name,d.supplierId,d.footprintName,d.uuid]);}catch(e){out[q]="ERR";}}
return out;
