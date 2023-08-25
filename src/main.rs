mod shapes;

use shapes::collisions::{Collidabale, Contains, Points};

use crate::shapes::{circle::Circle, rect::Rectangle};

use std::{fmt::Display, fs, str::FromStr};

enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rectangle(r) => return r.points(),
        }
    }
}

impl Contains for &Shape {
    fn contain_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contain_point(point),
            Shape::Rectangle(r) => return r.contain_point(point),
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));
        match shape {
            "Rectangle" => {
                return Ok(Shape::Rectangle(
                    data.parse::<Rectangle>()
                        .expect("wrong data for rectenagle"),
                ));
            }
            "Circle" => {
                return Ok(Shape::Circle(
                    data.parse::<Circle>().expect("wrong data for Circle"),
                ));
            }
            _ => {
                println!("Some problem ");
                return Result::Err(anyhow::anyhow!("Erorr"));
            }
        };
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // return write!(f, "Shape {}", self);
        match self {
            Shape::Circle(c) => write!(f, "{} ", c),
            Shape::Rectangle(r) => write!(f, "{} ", r),
        }
    }
}

fn main() {
    let shape = fs::read_to_string("shape")
        .expect("file not find")
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();
    let _collision_list = shape
        .iter()
        .skip(1)
        .zip(shape.iter().take(shape.len() - 1))
        .filter(|(a, b)| a.collide(b))
        .enumerate()
        .for_each(|(idx, (a, b))| println!("idx : {} -> {} collide with {}", idx, a, b));
    // .collect::<Vec<_>>();
}
