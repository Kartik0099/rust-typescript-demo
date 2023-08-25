use std::mem::ManuallyDrop;

struct Custom {
    age: usize,
    name: String,
}

union Item {
    age: usize,
    name: ManuallyDrop<String>,
    foo: ManuallyDrop<Custom>,
}

impl Custom {
    fn push(&self, age: usize, name: String) {}

    // fn print_self(&self){
    //     self.
    // }
}

// impl Item {
//     fn append(&self, age: usize, name: String) {
//         // match Item {
//         //     age => {}
//         //     name => {}
//         //     foo => {}
//         // }
//     }
// }

fn main() {
    let foo: Vec<Custom> = vec![Custom {
        age: 12,
        name: String::from("Hello"),
    }];

    // println!("{} {}", foo.age, foo.name);

    // foo.append(25, String::from("world"));

    // println!("{} {}", foo.age, foo.name);
}
