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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
/// Enum to configure interaction with the border of the canvas.
pub enum MovingBorderInteraction {
    /// Wrap around if the position exceeds the canvas.
    Wrap,
    /// Start moving into the other direction the moment the position exceeds the canvas.
    Reflect,
}
impl Default for MovingBorderInteraction {
    fn default() -> Self {
        MovingBorderInteraction::Wrap
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
/// Moves the first child (the kernel) over the second child or state's base canvas.
pub struct MovingKernel {
    pub vx: f64,
    pub vy: f64,

    pub x: f64,
    pub y: f64,

    // something with the border style... pass through, or reflect?
    #[serde(default)]
    pub border: MovingBorderInteraction,

    #[serde(skip)]
    children: Vec<EffectPtr>,
}
impl Effect for MovingKernel {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        // let mut canvas = state.get_canvas();

        let kernel = self.children[0].borrow_mut().update(state);
        let canvas: Canvas;
        if self.children.len() == 2 {
            canvas = self.children[1].borrow_mut().update(state);
        } else {
            canvas = state.get_canvas();
        }

        self.x += self.vx * state.get_elapsed();
        self.y += self.vy * state.get_elapsed();

        if self.border == MovingBorderInteraction::Wrap {
            if self.x < 0.0 {
                self.x = canvas.width() as f64;
            }
            if self.x > canvas.width() as f64 {
                self.x = 0.0
            }

            if self.y < 0.0 {
                self.y = canvas.height() as f64;
            }
            if self.y > canvas.height() as f64 {
                self.y = 0.0
            }
        }

        if self.border == MovingBorderInteraction::Reflect {
            if self.x < 0.0 || self.x > canvas.width() as f64 {
                self.vx *= -1.0
            }
            if self.y < 0.0 || self.y > canvas.width() as f64 {
                self.vy *= -1.0
            }
        }
        self.x = self.x.clamp(0.0, canvas.width() as f64);
        self.y = self.y.clamp(0.0, canvas.width() as f64);

        canvas.apply_onto(&kernel, self.x, self.y)
    }
    fn add_child(&mut self, effect: EffectPtr) {
        self.children.push(effect);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Makes a canvas that is the size of the specified rectangle, filled with the specified color.
pub struct Rectangle {
    pub width: usize,
    pub height: usize,

    pub color: RGBA,
}
impl Effect for Rectangle {
    fn update(&mut self, _state: &mut dyn State) -> Canvas {
        let mut out = Canvas::new(self.width, self.height);
        out.fill(&self.color);
        out
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
