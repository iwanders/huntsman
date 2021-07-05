use torch::effects::{Add, Effect, HorizontalMovingPixel, Retrieve, SetAlpha, Static, Store, Sub};
use torch::{BasicState, Canvas, RGBA};


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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    torch::loader::z();
    //pub fn load_effects(filename: &str) -> Result<EffectStorage, Box<dyn std::error::Error>>
    // let config = torch::loader::load_effects("./cfg/test_moving_pixels.json")?;
    // println!("pixels: {:?}", config);
    let config = torch::loader::load_effects("./cfg/test_moving_pixels.yaml")?;
    println!("yaml {:?}", config);

    let eff = torch::loader::make_effects_simple(&config.effects[..])?;

    // return Ok(());



    let mut mystate: BasicState = BasicState {
        stored: Default::default(),
        base_canvas: Canvas::transparent(23, 9),
        last_update_cycle: 0.0,
    };
    let ten_millis = std::time::Duration::from_millis(50);
    // while (true) {
    for i in 0..100 {
        let res = eff[0].borrow_mut().update(&mut mystate);
        println!("{}", res.to_string());
        // set_canvas(&mut h, &res);
        std::thread::sleep(ten_millis);
    }
    Ok(())
}
