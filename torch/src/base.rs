use std::ops;
#[derive(Clone, Copy, Debug, Default)]
pub struct RGBA {
    pub r: f64, // 0 is no red component, 1.0 is satured red component.
    pub g: f64,
    pub b: f64,
    pub a: f64, // 1 is completely visible, 0 is fully transparent
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
        (self.r * 255.0).round() as u8
    }
    pub fn g_u8(&self) -> u8 {
        (self.g * 255.0).round() as u8
    }
    pub fn b_u8(&self) -> u8 {
        (self.b * 255.0).round() as u8
    }
    pub fn a_u8(&self) -> u8 {
        (self.a * 255.0).round() as u8
    }

    pub fn with_alpha(&self, v: f64) -> RGBA {
        RGBA {
            a: v.clamp(0.0, 1.0),
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
        self.pixels
            .iter_mut()
            .map(|x| {
                *x = Default::default();
            })
            .collect()
    }

    pub fn pixel(&self, x: usize, y: usize) -> &RGBA {
        &self.pixels[y * self.width() + x]
    }
    pub fn pixel_as_mut(&mut self, x: usize, y: usize) -> &mut RGBA {
        let width = self.width();
        &mut self.pixels[y * width + x]
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
        for y in 0..self.height() {
            for x in 0..self.width() {
                let pix = &self.pixel(x, y);
                // make the color.
                out += format!("\x1b[38;2;{};{};{}m", pix.r_u8(), pix.g_u8(), pix.b_u8()).as_str();
                out += "\u{2588}\u{2588}"; // full block unicode character x 2
                out += "\x1b[0m"; // reset the color
            }
            out += "\n";
        }
        out
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
}
