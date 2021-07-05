pub use crate::base::{Canvas, State, RGBA};

use serde::{Deserialize, Serialize};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct NoConfig {}

pub type EffectPtr = Rc<RefCell<dyn Effect>>;
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
pub struct Add {
    pub children: Vec<EffectPtr>,
}
impl Effect for Add {
    fn add_child(&mut self, effect: EffectPtr) {
        self.children.push(effect);
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas {
        // first, retrieve all child updates.
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
pub struct Sub {
    pub children: Vec<EffectPtr>,
}
impl Effect for Sub {
    fn add_child(&mut self, effect: EffectPtr) {
        self.children.push(effect);
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas {
        // first, retrieve all child updates.
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
pub struct HorizontalMovingPixel {
    pub velocity: f64, // in pixels.
    pub row: usize,
    pub pixel: RGBA,
}
impl Effect for HorizontalMovingPixel {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut canvas = state.get_canvas();
        let t = state.get_time();
        let mut p = (self.velocity * t).abs() % ((canvas.width() - 1) as f64);
        if self.velocity < 0.0 {
            p = ((canvas.width() - 1) as f64) - p;
        }
        // Map float position to integers, always have two pixels illuminated with a ratio;
        let p0 = p.floor();
        let r = p - p0;

        *canvas.pixel_as_mut(p0 as usize, self.row) = self.pixel.with_alpha(1.0 - r);
        *canvas.pixel_as_mut((p0 as usize) + 1, self.row) = self.pixel.with_alpha(r);
        canvas
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

        let alpha_factor = if (self.scale_by_time && self.scale_alpha) {
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
