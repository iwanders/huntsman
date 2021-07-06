use huntsman::RGB;
use torch::{BasicState, Canvas, State};


fn get_time() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    return (since_the_epoch.as_secs() as f64)
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000_000.0);
}


#[allow(dead_code)]
fn set_canvas(h: &mut huntsman::Huntsman, c: &Canvas) -> Result<(), String> {
    let s = get_time();
    let mut rgb_buff: Vec<RGB> = vec![Default::default(); c.width()];
    for y in 0..c.height() {
        for x in 0..usize::min(c.width(), 23) {
            rgb_buff[x].r = c.pixel(x, y).r_u8();
            rgb_buff[x].g = c.pixel(x, y).g_u8();
            rgb_buff[x].b = c.pixel(x, y).b_u8();
        }
        h.set_color(y as u8, &rgb_buff[..])?
    }
    println!("set canvas took: {:.5}", get_time() - s);
    Ok(())
}

#[allow(unreachable_code, unused_variables)]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let config = torch::loader::load_effects(&args[1])?;
    println!("yaml {:?}", config);

    let eff = torch::loader::make_effects_simple(&config.effects[..])?;

    // return Ok(());

    let mut mystate: BasicState = BasicState {
        stored: Default::default(),
        base_canvas: Canvas::transparent(23, 9),
        last_update_cycle: 0.0,
    };
    mystate.start_update();
    mystate.finish_update();
    let ten_millis = std::time::Duration::from_millis(50);

    // let mut h = huntsman::Huntsman::new()?;
    // h.effect_custom()?;
    // loop {
    for _i in 0..100 {
        mystate.start_update();
        let s = get_time();
        let res = eff[0].borrow_mut().update(&mut mystate);
        // println!("update took: {:.5}", get_time() - s);
        mystate.finish_update();
        println!("{}", res.to_string());
        // set_canvas(&mut h, &res)?;
        std::thread::sleep(ten_millis);
    }
    Ok(())
}
