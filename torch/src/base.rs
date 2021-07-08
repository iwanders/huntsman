use serde::{Deserialize, Serialize};
use std::ops;
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]

/// Representation of a pixel with an alpha channel.
pub struct RGBA {
    /// Red component, 0 is no contribution, 1.0 is saturated red.
    pub r: f64,
    pub g: f64,
    pub b: f64,
    /// Alpha component, 1 is completely visible, 0 is fully transparent
    pub a: f64,
}
impl RGBA {
    pub fn red() -> RGBA {
        RGBA {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
    pub fn green() -> RGBA {
        RGBA {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        }
    }
    pub fn blue() -> RGBA {
        RGBA {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
    }
    pub fn white() -> RGBA {
        RGBA {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
    pub fn transparent() -> RGBA {
        RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
    pub fn opaque() -> RGBA {
        RGBA {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }

    pub fn clamp(&mut self) {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
        self.a = self.a.clamp(0.0, 1.0);
    }

    pub fn r_u8(&self) -> u8 {
        (self.r * self.a * 255.0).round() as u8
    }
    pub fn g_u8(&self) -> u8 {
        (self.g * self.a * 255.0).round() as u8
    }
    pub fn b_u8(&self) -> u8 {
        (self.b * self.a * 255.0).round() as u8
    }
    pub fn a_u8(&self) -> u8 {
        (self.a * self.a * 255.0).round() as u8
    }

    pub fn with_alpha(&self, v: f64) -> RGBA {
        RGBA {
            a: v.clamp(0.0, 1.0),
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
    pub fn scaled_alpha(&self, v: f64) -> RGBA {
        RGBA {
            a: self.a * v.clamp(0.0, 1.0),
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    pub fn set_alpha(&mut self, v: f64) {
        self.a = v.clamp(0.0, 1.0);
    }
}

impl ops::Add<RGBA> for RGBA {
    type Output = RGBA;
    fn add(self, rhs: RGBA) -> RGBA {
        let mut out: RGBA = Default::default();
        out.r = self.r * self.a + rhs.r * rhs.a;
        out.g = self.g * self.a + rhs.g * rhs.a;
        out.b = self.b * self.a + rhs.b * rhs.a;
        out.a = self.a + rhs.a;
        out.clamp();
        out
    }
}

impl ops::Sub<RGBA> for RGBA {
    type Output = RGBA;
    fn sub(self, rhs: RGBA) -> RGBA {
        let mut out: RGBA = Default::default();
        out.r = self.r * self.a - rhs.r * rhs.a;
        out.g = self.g * self.a - rhs.g * rhs.a;
        out.b = self.b * self.a - rhs.b * rhs.a;
        out.a = self.a - rhs.a;
        out.clamp();
        out
    }
}

impl ops::Mul<f64> for RGBA {
    type Output = RGBA;
    fn mul(self, rhs: f64) -> RGBA {
        let mut out = self;
        out.r *= rhs;
        out.g *= rhs;
        out.b *= rhs;
        out.a *= rhs;
        out.clamp();
        out
    }
}

#[derive(Clone, Debug)]
/// A rectangular grid of pixels makes up a canvas. Canvas coordinate system is y positive is up,
/// x positive is to the right. So 0,0 is the bottom left corner.
pub struct Canvas {
    canvas_width: usize,
    canvas_height: usize,
    pixels: Vec<RGBA>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            canvas_width: width,
            canvas_height: height,
            pixels: vec![Default::default(); width * height],
        }
    }
    pub fn transparent(width: usize, height: usize) -> Canvas {
        Canvas {
            canvas_width: width,
            canvas_height: height,
            pixels: vec![RGBA::transparent(); width * height],
        }
    }

    pub fn wipe(&mut self) {
        self.fill(&Default::default())
    }

    pub fn fill(&mut self, color: &RGBA) {
        self.pixels
            .iter_mut()
            .map(|x| {
                *x = *color;
            })
            .collect()
    }

    pub fn pixel(&self, x: usize, y: usize) -> &RGBA {
        &self.pixels[(self.height() - y - 1) * self.width() + x]
    }
    pub fn pixel_as_mut(&mut self, x: usize, y: usize) -> &mut RGBA {
        let width = self.width();
        let height = self.height();
        &mut self.pixels[(height - y - 1) * width + x]
    }

    pub fn within(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }

    pub fn width(&self) -> usize {
        self.canvas_width
    }
    pub fn height(&self) -> usize {
        self.canvas_height
    }
    pub fn iter(&self) -> std::slice::Iter<RGBA> {
        self.pixels.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<RGBA> {
        self.pixels.iter_mut()
    }

    pub fn to_string(&self) -> String {
        //:~$ printf "\x1b[38;2;0;255;255mTRUECOLOR\x1b[0m\n"
        // https://stackoverflow.com/a/26665998h
        let mut out = String::new();
        for y in (0..self.height()).rev() {
            for x in 0..self.width() {
                let pix = &self.pixel(x, y);
                // make the color.
                out += format!("\x1b[38;2;{};{};{}m", pix.r_u8(), pix.g_u8(), pix.b_u8()).as_str();
                out += "\u{2588}\u{2588}"; // full block unicode character x 2
                out += "\x1b[0m"; // reset the color
            }
            if y != 0 {
                out += "\n";
            }
        }
        out
    }

    pub fn apply_onto(&self, base: &Canvas, x: f64, y: f64) -> Canvas {
        let mut res = self.clone();

        // Each pixel maps to 4 other pixels, this is always true.
        let x_0: usize = x.floor() as usize;
        let x_r: f64 = x - x_0 as f64;
        let y_0: usize = y.floor() as usize;
        let y_r: f64 = y - y_0 as f64;
        //  (x0, y0)     (x0+1, y0)
        //          +----+----+
        //          |    |    |
        //          +----*----+
        //          |    |    |
        //          +----+----+
        // *: (x0 + 1, y0 + 1)

        for ky in 0..base.height() {
            for kx in 0..base.width() {
                let base_val = *base.pixel(kx, ky);
                if res.within(x_0 + kx + 1, y_0 + ky + 1) {
                    let current = *res.pixel(x_0 + kx + 1, y_0 + ky + 1);
                    *res.pixel_as_mut(x_0 + kx + 1, y_0 + ky + 1) =
                        current + base_val.scaled_alpha((x_r) * (y_r));
                }
                if res.within(x_0 + kx + 0, y_0 + ky + 1) {
                    let current = *res.pixel(x_0 + kx + 0, y_0 + ky + 1);
                    *res.pixel_as_mut(x_0 + kx + 0, y_0 + ky + 1) =
                        current + base_val.scaled_alpha((1.0 - x_r) * (y_r));
                }
                if res.within(x_0 + kx + 1, y_0 + ky + 0) {
                    let current = *res.pixel(x_0 + kx + 1, y_0 + ky + 0);
                    *res.pixel_as_mut(x_0 + kx + 1, y_0 + ky + 0) =
                        current + base_val.scaled_alpha((x_r) * (1.0 - y_r));
                }
                if res.within(x_0 + kx + 0, y_0 + ky + 0) {
                    let current = *res.pixel(x_0 + kx + 0, y_0 + ky + 0);
                    *res.pixel_as_mut(x_0 + kx + 0, y_0 + ky + 0) =
                        current + base_val.scaled_alpha((1.0 - x_r) * (1.0 - y_r));
                }
            }
        }
        res
    }
}

impl ops::Add<Canvas> for Canvas {
    type Output = Canvas;
    fn add(self, rhs: Canvas) -> Canvas {
        let mut res = self.clone();
        res.wipe();
        let _ = self
            .iter()
            .zip(rhs.iter())
            .zip(res.iter_mut())
            .map(|((l, r), out)| {
                *out = *l + *r;
            })
            .collect::<Vec<_>>();
        res
    }
}

impl ops::Sub<Canvas> for Canvas {
    type Output = Canvas;
    fn sub(self, rhs: Canvas) -> Canvas {
        let mut res = self.clone();
        res.wipe();
        let _ = self
            .iter()
            .zip(rhs.iter())
            .zip(res.iter_mut())
            .map(|((l, r), out)| {
                *out = *l - *r;
            })
            .collect::<Vec<_>>();
        res
    }
}

/// Holder object for state that we want to share, like time.
pub trait State {
    /// Get a named canvas from the state.
    fn get_stored(&self, name: &str) -> Option<Canvas>;

    fn set_stored(&mut self, name: &str, canvas: Canvas);

    /// Retrieve the time as a float value.
    fn get_time(&self) -> f64;

    /// Retrieve an appropriately sized canvas
    fn get_canvas(&self) -> Canvas;

    /// Retrieve the time elapsed since last update cycle.
    fn get_elapsed(&self) -> f64;

    /// Method to tell the state a new update cycle is starting.
    fn start_update(&mut self) -> () {}

    /// Function to tell the state an update cycle has finished
    fn finish_update(&mut self) -> () {}

    /// Return an integer representing how many times start_update has been called.
    fn get_update(&self) -> usize { 0 }

    /// Obtain the random number generator.
    fn get_rng(&mut self) -> &mut rand::rngs::ThreadRng;
}
