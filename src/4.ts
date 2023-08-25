// const list = [1,2,3,];


function a(list:number[], index:number):Number | undefined{
    return (list[index] || index)*5
}

console.log(a([1,2,5], 7));


