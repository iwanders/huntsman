use crate::base::{Canvas, State, RGBA};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

pub type EffectPtr = Rc<RefCell<dyn Effect>>;
/// Effect trait that can manipulate colors and output a canvas.
pub trait Effect: std::fmt::Debug {
    fn get_name(&self) -> String {
        "Unnamed".to_owned()
    }

    fn add_child(&mut self, _effect: EffectPtr) {
        panic!("No add child functionality for this effect.");
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas;
}
pub fn make_effect<T: 'static + Effect + Sized>(v: T) -> EffectPtr {
    Rc::new(RefCell::new(v))
}

#[derive(Debug)]
/// Addition operation, adds any children together.
pub struct Add {
    pub children: Vec<EffectPtr>,
}
impl Effect for Add {
    fn add_child(&mut self, effect: EffectPtr) {
        self.children.push(effect);
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut child_states = self
            .children
            .iter_mut()
            .map(|x| x.borrow_mut().update(state))
            .collect::<Vec<Canvas>>();
        let out = child_states.pop();
        if out.is_none() {
            panic!("Addition with no children.");
        }
        let mut out = out.unwrap();
        for rhs in child_states.drain(..) {
            out = out + rhs;
        }
        out
    }
}
impl Add {
    pub fn new() -> EffectPtr {
        make_effect(Add { children: vec![] })
    }
}

#[derive(Debug)]
/// Subtraction operation, takes the first child and removes all others from it.
pub struct Sub {
    pub children: Vec<EffectPtr>,
}
impl Effect for Sub {
    fn add_child(&mut self, effect: EffectPtr) {
        self.children.push(effect);
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut child_states = self
            .children
            .iter_mut()
            .map(|x| x.borrow_mut().update(state))
            .collect::<Vec<Canvas>>();
        if child_states.len() == 0 {
            panic!("Addition with no children.");
        }
        let mut out = child_states.remove(0);
        for rhs in child_states.drain(..) {
            out = out - rhs;
        }
        out
    }
}
impl Sub {
    pub fn new() -> EffectPtr {
        make_effect(Sub { children: vec![] })
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
/// Makes a moving horizontal pixel.
pub struct HorizontalMovingPixel {
    pub velocity: f64, // in pixels.
    pub row: usize,
    pub color: RGBA,
    // #[serde(default)]
    // pub position: f64,
}
impl Effect for HorizontalMovingPixel {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut canvas = state.get_canvas();

        let mut kernel = Canvas::new(1, 1);
        *kernel.pixel_as_mut(0, 0) = self.color;

        let t = state.get_time();
        let mut p = (self.velocity * t).abs() % ((canvas.width() - 1) as f64);
        if self.velocity < 0.0 {
            p = ((canvas.width() - 1) as f64) - p;
        }

        canvas.apply_onto(&kernel, p, 0.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Retrieves a canvas by name from the state's storage.
pub struct Retrieve {
    pub name: String,
}
impl Effect for Retrieve {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let res = state.get_stored(&self.name);
        if res.is_none() {
            return state.get_canvas();
        }
        res.unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Stores the retrieved canvas of the only child, returns stored value.
pub struct Store {
    pub name: String,
    #[serde(skip)]
    pub child: Option<EffectPtr>,
}
impl Effect for Store {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let canvas = self.child.as_mut().unwrap().borrow_mut().update(state);
        state.set_stored(&self.name, canvas.clone());
        canvas
    }
    fn add_child(&mut self, effect: EffectPtr) {
        self.child = Some(effect);
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
/// Fill a canvas with a static value.
pub struct Static {
    pub color: RGBA,

    #[serde(default)]
    /// Scale the static value such that in 1 time unit it would accumulate to the requested value.
    pub scale_by_time: bool,

    #[serde(default)]
    /// If this is true, alpha is also scaled, otherwise this is kept at the original value.
    pub scale_alpha: bool,
}
impl Effect for Static {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut canvas = state.get_canvas();
        println!("Elapsed: {} t: {}", state.get_elapsed(), self.scale_by_time);
        let scale_factor = if self.scale_by_time {
            state.get_elapsed()
        } else {
            1.0
        };

        let alpha_factor = if self.scale_by_time && self.scale_alpha {
            state.get_elapsed()
        } else {
            self.color.a
        };

        for p in canvas.iter_mut() {
            *p = (self.color * scale_factor).with_alpha(alpha_factor);
        }
        canvas
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Set the alpha of the child canvas that's retrieved.
pub struct SetAlpha {
    #[serde(skip)]
    pub child: Option<EffectPtr>,
    pub value: f64,
}

impl Effect for SetAlpha {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut canvas = self.child.as_mut().unwrap().borrow_mut().update(state);
        for p in canvas.iter_mut() {
            p.set_alpha(self.value);
        }
        canvas
    }
    fn add_child(&mut self, effect: EffectPtr) {
        self.child = Some(effect);
    }
}

#[cfg(test)]
mod tests {
    struct DummyState {
        pub time: f64,
        pub elapsed: f64,
        pub canvas: Canvas,
    }
    impl State for DummyState {
        fn get_stored(&self, _name: &str) -> Option<Canvas> {
            None
        }

        fn set_stored(&mut self, _name: &str, _canvas: Canvas) {}

        fn get_time(&self) -> f64 {
            self.time
        }

        fn get_canvas(&self) -> Canvas {
            self.canvas.clone()
        }

        fn get_elapsed(&self) -> f64 {
            self.elapsed
        }
    }

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_moving_pixel() {
        let mut state: DummyState = DummyState {
            time: 1.0,
            elapsed: 0.0,
            canvas: Canvas::new(10, 1),
        };
        let mut eff: HorizontalMovingPixel = HorizontalMovingPixel {
            color: RGBA::red(),
            velocity: 1.0,
            row: 0,
        };

        println!("\n{}", eff.update(&mut state).to_string());
        state.time += 0.5;
        println!("{}", eff.update(&mut state).to_string());
        state.time += 0.5;
        println!("{}", eff.update(&mut state).to_string());
        state.time += 0.25;
        println!("{}", eff.update(&mut state).to_string());
    }

    #[test]
    fn test_kernel_blend_single() {
        let out = Canvas::new(5, 5);
        let mut kernel = Canvas::new(1, 1);
        kernel.pixel_as_mut(0, 0).r = 1.0;
        kernel.pixel_as_mut(0, 0).a = 1.0;
        println!("kernel: \n{}", kernel.to_string());

        println!("{}", out.apply_onto(&kernel, 0.0, 0.0).to_string());
        println!("{}", out.apply_onto(&kernel, 1.0, 0.0).to_string());
        println!("{}", out.apply_onto(&kernel, 0.0, 1.0).to_string());
        println!("{}", out.apply_onto(&kernel, 1.0, 1.0).to_string());
        println!("{}", out.apply_onto(&kernel, 0.5, 0.0).to_string());
        println!("{}", out.apply_onto(&kernel, 0.5, 0.5).to_string());
    }

    #[test]
    fn test_kernel_blend_square() {
        let out = Canvas::new(5, 5);
        let mut kernel = Canvas::new(2, 2);
        *kernel.pixel_as_mut(0, 0) = RGBA::green();
        *kernel.pixel_as_mut(1, 1) = RGBA::green();
        *kernel.pixel_as_mut(0, 1) = RGBA::green();
        *kernel.pixel_as_mut(1, 0) = RGBA::green();

        println!("kernel: \n{}", kernel.to_string());

        println!("{}", out.apply_onto(&kernel, 0.5, 0.0).to_string());
        println!("{}", out.apply_onto(&kernel, 0.0, 0.5).to_string());
        println!("{}", out.apply_onto(&kernel, 0.75, 0.0).to_string());
        println!("{}", out.apply_onto(&kernel, 0.0, 0.75).to_string());
        println!("{}", out.apply_onto(&kernel, 1.0, 1.0).to_string());
        println!("{}", out.apply_onto(&kernel, 3.5, 3.5).to_string());
    }
}
