import fs from "fs";

fs.readFileSync("lines")
  .toString()
  .split("\n")
  .filter((_line, idx) => idx % 2 == 0)
  .filter((_,idx)=>idx>1 && idx<4)
  .forEach((line) => {
      console.log(line);
  }); 

// console.log("a : ",a )

// let foo:string = a +  "\nhello\nworld"

// fs.writeFileSync("lines",foo);

// fs.readFileSync("lines").toString().split("\n").forEach(line=>console.log(line));
