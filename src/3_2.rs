#[allow(dead_code)]

struct Custom {
    age: u16,
    name: String,
}

enum Item {
    Number(u16),
    String(String),
    #[allow(dead_code)]
    MyCustom(Custom),
}

#[allow(dead_code)]
fn append(items: &mut Vec<Item>) -> () {
    items.push(Item::String("Hello world".to_owned()));
}

fn patten_match(item: &Item) {
    match item {
        Item::MyCustom(Custom { name, .. }) => println!("name : {}", name),
        #[allow(unreachable_patterns)]
        Item::MyCustom(Custom { age, .. }) => println!("Age : {}", age),

        Item::Number(num) => println!("Num {}", num),
        Item::String(str) => println!("Sting {}", str),
    }
}

fn main() {
    // let mut foo: Vec<Item> = vec![];

    // append(&mut foo);

    // let mut bar: Vec<u16> = vec![];

    // append(bar);

    let foo: Custom = Custom {
        age: 25,
        name: "hello".to_owned(),
    };

    patten_match(&Item::MyCustom(foo));

    let baz: Item = Item::MyCustom(Custom {
        age: 32,
        name: "Hello Rust".to_owned(),
    });

    patten_match(&baz);

    // let foo: Custom = Custom {
    //     age: 25,
    //     name: "hello".to_owned(),
    // };

    // patten_match(&Item::MyCustom());

    let bar: Item = Item::Number(5);

    // println!("number L: ");

    patten_match(&bar);
}
