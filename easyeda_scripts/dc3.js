const nl=await eda.sch_Netlist.getNetlist("Protel2");
return {dc1_1:nl.includes("DC1-1 "),dc1_2:nl.includes("DC1-2 "),dc1_3:nl.includes("DC1-3 ")};
