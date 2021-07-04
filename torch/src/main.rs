use torch::effects::{Add, Sub, Effect, Static, HorizontalMovingPixel, Store, Retrieve, SetAlpha};
use torch::{BasicState, RGBA, Canvas};

use huntsman::RGB;

fn set_canvas(h: &mut huntsman::Huntsman, c: &Canvas) -> Result<(), String> {
    let mut rgb_buff: Vec<RGB> = vec![Default::default(); c.width()];
    for y in 0..c.height() {
        for x in 0..usize::min(c.width(), 23) {
            rgb_buff[x].r = c.pixel(x, y).r_u8();
            rgb_buff[x].g = c.pixel(x, y).g_u8();
            rgb_buff[x].b = c.pixel(x, y).b_u8();
        }
        h.set_color(y as u8, &rgb_buff[..])?
    }
    Ok(())
}

pub fn main() -> Result<(), String> {
    let mut c1 = Canvas::new(10, 10);
    *c1.pixel_as_mut(0, 0) = RGBA::red();
    let mut c2 = Canvas::new(10, 10);
    *c2.pixel_as_mut(0, 0) = RGBA::blue();

    println!("{}", c1.to_string());
    println!("{}", c2.to_string());
    let c1_plus_c2 = c1 + c2;

    println!("{}", c1_plus_c2.to_string());

    let mut mystate: BasicState = BasicState {
        stored: Default::default(),
        base_canvas: Canvas::transparent(23, 9),
        last_update_cycle: 0.0,
    };
    let pixel_0: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        velocity: 5.0,
        row: 0,
        pixel: RGBA::red(),
    });
    let pixel_1: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        velocity: 2.0,
        row: 2,
        pixel: RGBA::red(),
    });
    let pixel_2: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        velocity: 0.5,
        row: 3,
        pixel: RGBA::red(),
    });
    let pixel_3: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        velocity: -5.0,
        row: 4,
        pixel: RGBA::red(),
    });

    // let pixel_b1: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        // velocity: 1.0,
        // row: 1,
        // pixel: RGBA::blue(),
    // });
    // let pixel_b2: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        // velocity: 1.0,
        // row: 2,
        // pixel: RGBA::blue(),
    // });
    // let pixel_b3: Box<dyn Effect> = Box::new(HorizontalMovingPixel {
        // velocity: 1.0,
        // row: 3,
        // pixel: RGBA::blue(),
    // });
    let mut add = Box::new(Add { children: vec![] });

    let mut history_make_opaque = Box::new(SetAlpha{value: 1.0, child: None});
    history_make_opaque.add_child(Box::new(Retrieve{name: "stored".to_string()}));

    let mut history_decayed = Box::new(Sub{children: vec!()});
    history_decayed.add_child(history_make_opaque);
    history_decayed.add_child(Box::new(Static{color: (RGBA::white() * 0.1).with_alpha(1.0)}));
    let mut history_decayed_opaque = Box::new(SetAlpha{value: 1.0, child: None});
    history_decayed_opaque.add_child(history_decayed);

    add.add_child(history_decayed_opaque);
    add.add_child(pixel_0);
    add.add_child(pixel_1);
    add.add_child(pixel_2);
    add.add_child(pixel_3);

    for i in 0..9 {
        add.add_child(Box::new(HorizontalMovingPixel {
            velocity: 1.0,
            row: i,
            pixel: RGBA::blue(),
        }));
    }

    let mut store: Box<Effect> = Box::new(Store{name: "stored".to_string(), child:None});
    store.add_child(add);
    // add.add_child(pixel_b1);
    // add.add_child(pixel_b3);
    let entry = &mut store;


    use std::{thread, time};

    // let mut h = huntsman::Huntsman::new()?;
    // h.effect_custom()?;

    let ten_millis = time::Duration::from_millis(50);
    // while (true) {
    for i in 0..100 {
        let res = entry.update(&mut mystate);
        println!("{}", res.to_string());
        // set_canvas(&mut h, &res);
        thread::sleep(ten_millis);
    }
    Ok(())
}
