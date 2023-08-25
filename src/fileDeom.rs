use std::{env::args, fs};

fn main() {
    let args: String = args().nth(2).expect("The File name to be passed");

    println!("{:?}", args);

    let a = fs::read_to_string(args).expect("unable to read file");

    a.lines().for_each(|x| println!("{}", x));

    // for line in a.chars() {
    //     if line == '\n' {
    //         println!();
    //         continue;
    //     }
    //     print!("{}", line);
    // }
}
