fn mul(list: Option<u16>, index: u16) -> Option<u16> {
    return Some(list?) * 5;
}

fn mul(list: Vec<usize>, index: usize) -> usize {
    // if let Some(x) = list.get(index) {
    //     x * 5
    // }

    return list.get(index).unwrap_or(&index) * 5;
}

fn main() {
    println!("value : {}", mul(None, 5));
    println!("value : {}", mul(vec![1, 2, 3, 4, 5, 6], 10));
}
