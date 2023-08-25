enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: &Color) {
    match color {
        Color::Red => {
            println!("Red");
        }
        Color::Green => {
            println!("Green");
        }
        Color::Blue => {
            println!("Blue");
        }
        Color::Yellow => println!("yellow"),
    }
}

fn main() {
    let color = Color::Yellow;
    print_color(&color);

    // let color = Color::Yellow;

    println!("Color Blue : {:?}", color.is_green_parts());
    println!("Color Green : {:?}", color.is_green());
}
