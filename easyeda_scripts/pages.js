const S="10cb76caae5eb1a8";
let pages=(await eda.dmt_Schematic.getSchematicInfo(S)).page;
while(pages.length<3){await eda.dmt_Schematic.createSchematicPage(S);await new Promise(r=>setTimeout(r,800));pages=(await eda.dmt_Schematic.getSchematicInfo(S)).page;}
const names=["MAIN_CONTROLLER","SENSOR_PATCH","SPLITTER"];
for(let i=0;i<3;i++) await eda.dmt_Schematic.modifySchematicPageName(pages[i].uuid,names[i]);
pages=(await eda.dmt_Schematic.getSchematicInfo(S)).page;
return pages.map(p=>[p.uuid,p.name]);
