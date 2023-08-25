#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_count(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    items.iter().for_each(|x| println!("{:?}", x));
}

fn main() {
    let mut item: Item = Item { count: 0 };

    add_count(&mut item);

    let mut items: Vec<Item> = vec![item];
    items.push(Item { count: 6 });
    // Item { count: 5 }, Item { count: 6 }

    let first_ref = items.first_mut();

    println!("{:?}", first_ref);
    if let Some(x) = first_ref {
        println!("{:?}", x);
    }

    print_all(&items);

    let list = vec![1, 2, 3];

    let foo = list.iter().map(|x| x + 1);

    println!("{:?}", foo);

    // println!("{:?}", first_ref);

    // println!("{:?}", item);

    // println!("{:?}", item);
}
