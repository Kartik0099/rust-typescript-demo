fn main() {
    // let _list: Vec<i32> = vec![1, 2, 3]
    //     .iter()
    //     .map(|&x| {
    //         return x + 1;
    //     })
    //     .collect();

    // println!("{:?}", _list);

    // let data: Vec<_> = vec![1, 2, 3];

    // let mut foo = data.iter().map(|&x| &x + 1);

    // let mut bar: Vec<i32> = vec![];

    // while let Some(x) = foo.next() {
    //     bar.push(x);
    // }

    // println!("{:?}", bar);

    let mut foo = vec![1, 2, 3]
        .into_iter()
        .filter(|x| x % 2 != 0)
        .map(|x| x + 1);

    let mut bar = Vec::new();

    while let Some(x) = foo.next() {
        bar.push(x);
    }

    println!("{:?}", bar);
}
