use torch::{Canvas, RGBA, BasicState, Effect, AdditionEffect, HorizontalMovingRedPixel};

pub fn main()
{
    let mut c1 = Canvas::new(10, 10);
    *c1.pixel_as_mut(0, 0) = RGBA::red();
    let mut c2 = Canvas::new(10, 10);
    *c2.pixel_as_mut(0, 0) = RGBA::blue();

    println!("{}", c1.to_string());
    println!("{}", c2.to_string());
    let c1_plus_c2 = c1 + c2;

    println!("{}", c1_plus_c2.to_string());


    let mut mystate: BasicState = BasicState{ stored: Default::default(), base_canvas: Canvas::new(23, 8)};
    let pixel_1: Box<dyn Effect> = Box::new(HorizontalMovingRedPixel{ velocity: 2.0, row: 2 });
    let pixel_2: Box<dyn Effect> = Box::new(HorizontalMovingRedPixel{ velocity: 0.5, row: 3 });
    let mut add = Box::new(AdditionEffect{children: vec!()});

    add.add_child(pixel_1);
    add.add_child(pixel_2);

    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(50);
    for _i in 0..4000
    {
        let res = add.update(&mut mystate);
        println!("{}", res.to_string());
        thread::sleep(ten_millis);
    }
}
