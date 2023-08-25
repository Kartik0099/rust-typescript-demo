use std::{fmt::Display, str::FromStr};

use super::{
    area::Area,
    // circle::Circle,
    collisions::{Contains, Points},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Contains for Rectangle {
    // type x = f64;
    // type y = f64;
    // fn new(x:f64,y,width,height)->Self{
    //     return Rectangle { x, y, width, height }
    // }
    fn contain_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && x <= self.x + self.width && self.y <= y && y <= self.y + self.height;
    }
}

impl Points for Rectangle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
        ]
        .into();
    }
}

impl FromStr for Rectangle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("bad Rectangle Points from the strings"));
        }

        return Ok(Rectangle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}
// impl Collidabale<Rectangle> for Rectangle {
//     fn collide(&self, other: &Rectangle) -> bool {
//         for point in other {
//             if self.contain_point(point) {
//                 return true;
//             }
//         }

//         return false;
//     }
// }

// impl Collidabale<Circle> for Rectangle {
//     // here our algo is wrong only checking center is on rect or not
//     fn collide(&self, other: &Circle) -> bool {
//         return self.contain_point((other.x, other.y));
//     }
// }

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            width: 10.0,
            height: 20.0,
            x: 10.0,
            y: 50.0,
        };
    }

    // fn default_value(x: f64, y: f64, height: f64, width: f64) -> Self {
    //     return Rectangle {
    //         x,
    //         y,
    //         height,
    //         width,
    //     };
    // }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // \n...........END..........\n"
        return write!(
            f,
            "Rectangle x:{} y:{} width: {} height: {} Area: {} ",
            self.x,
            self.y,
            self.width,
            self.height,
            self.area()
        );
    }
}

// pub struct RectIter {
//     points: Vec<(f64, f64)>,
//     index: usize,
// }

// impl RectIter {
//     fn new(rect: &Rectangle) -> Self {
//         return RectIter {
//             points: vec![
//                 (rect.x, rect.y),
//                 (rect.x + rect.width, rect.y),
//                 (rect.x, rect.y + rect.height),
//                 (rect.x + rect.width, rect.y + rect.height),
//             ],
//             index: 0,
//         };
//     }
// }

// impl From<&Rectangle> for RectIter {
//     fn from(value: &Rectangle) -> Self {
//         return RectIter {
//             points: vec![
//                 (value.x, value.y),
//                 (value.x + value.width, value.y),
//                 (value.x, value.y + value.height),
//                 (value.x + value.width, value.y + value.height),
//             ],
//             index: 0,
//         };
//     }
// }

// impl Into<&Rectangle> for RectIter {
//     fn into(self) -> &Rectangle {
//         return ;
//     }
// }

// impl Iterator for RectIter {
//     type Item = (f64, f64);

//     fn next(&mut self) -> Option<Self::Item> {
//         // if self.index >= self.points.len() {
//         //     return None;
//         // }
//         // let point: (f64, f64) = self.points[self.index];
//         // self.index += 1;
//         // return Some(point);
//         let idx = self.index;
//         self.index += 1;

//         return self.points.get(idx).map(|x| *x);
//     }
// }

// impl IntoIterator for Rectangle {
//     type Item = (f64, f64);

//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         // let rect: &Rectangle = self.into();
//         // return rect;
//         // return RectIter::new(&self);
//         return (&self).into();
//     }
// }

// impl IntoIterator for &Rectangle {
//     type Item = (f64, f64);

//     type IntoIter = RectIter;

//     fn into_iter(self) -> Self::IntoIter {
//         return self.into();
//         // return RectIter::new(self);
//     }
// }
