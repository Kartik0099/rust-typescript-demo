import fs from 'fs';
import { join } from 'path';

let str =  process.argv[2]?.toString();

let fileName = "fi";


if(str){

    fs.writeFileSync(join(__dirname,fileName),str,{flag:'w',encoding:'utf-8'});

    let contents = fs.readFileSync(join(__dirname,fileName),{encoding:'utf-8'});

    console.log("Content : ",contents);
}
