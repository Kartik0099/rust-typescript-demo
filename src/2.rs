// #[warn(dead_code)]
fn lines_print(contents: String) -> () {
    contents
        .lines()
        .enumerate()
        .filter(|(i, _x)| {
            return *i % 2 == 0;
        })
        .skip(2)
        .take(2)
        .for_each(|line: (usize, &str)| println!("{}", line.1));
}

fn main() {
    // let a = "hello\nfem\nhow\n1\n2\n3\nare\n you?\nhello\nworld\nrust\nworld\n";

    let contents = std::fs::read_to_string("lines");
    // lines_print(contents);

    match contents {
        Ok(_contents) => lines_print(_contents),
        Err(_err) => {
            println!("Got error");
        }
    }

    // let mut _write_data: String = contents + "\nrust\nworld";

    // std::fs::write("lines", _write_data).unwrap();

    // let mut _contents: String = std::fs::read_to_string("lines").unwrap();

    // _contents.lines().for_each(|line| println!("{}", line));
}
