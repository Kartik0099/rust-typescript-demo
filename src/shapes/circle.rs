use std::{f64::consts::PI, fmt::Display, str::FromStr};

use super::{
    area::Area,
    collisions::{Contains, Points},
    // rect::Rectangle,
};

#[allow(dead_code)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Contains for Circle {
    fn contain_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![(self.x, self.y)].into();
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("bad Circle Points from the strings"));
        }

        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle : x:{} y:{} A:{} ", self.x, self.y, self.radius);
    }
}
// impl Collidabale<Rectangle> for Circle {
//     fn collide(&self, other: &Rectangle) -> bool {
//         for point in other {
//             if self.contain_point(point) {
//                 return true;
//             }
//         }

//         return false;
//     }
// }

// impl Collidabale<Circle> for Circle {
//     // here our algo is wrong only checking center is on rect or not
//     fn collide(&self, other: &Circle) -> bool {
//         return self.contain_point((other.x, other.y));
//     }
// }

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}
