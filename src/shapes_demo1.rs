mod shapes;
#[allow(unused_imports)]
use crate::shapes::{area::Area, circle::Circle, collisions::Collidabale, rect::Rectangle};

fn abc() {
    // let r = Rectangle::default();

    // let c = Circle {
    //     radius: 10.0,
    //     x: 10.0,
    //     y: 50.0,
    // };

    // println!("{}", r);
    // // println!("Rect Area L:{:?}", r.area());

    // println!("Circle Area L:{:?}", c.area());

    // println!("Circle Area L:{:?}", 1.1.area());

    let r1 = Rectangle::default();

    let r2 = Rectangle {
        x: 10.0,
        y: 15.0,
        width: 30.0,
        height: 30.0,
    };

    let c1 = Circle {
        x: 10.0,
        y: 10.0,
        radius: 5.0,
    };

    let c2 = Circle {
        x: 10.0,
        y: 15.0,
        radius: 15.0,
    };

    println!("r1 collide r2 {}", r1.collide(&r2));

    println!("c1 collide c2 {}", c1.collide(&c2));

    println!("r1 collide c1 {}", r1.collide(&c1));

    println!("r2 collide c2 {}", r2.collide(&c2));

    // println!("Rectagele Points");
    // for point in &r1 {
    //     println!("{:?}", point)
    // }
    // println!("\n");

    // println!("{:?}", r1);
}
