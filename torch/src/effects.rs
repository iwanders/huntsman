pub use crate::base::{Canvas, State, RGBA};

pub trait Effect: std::fmt::Debug {
    fn get_name(&self) -> String {
        "Unnamed".to_owned()
    }

    fn add_child(&mut self, effect: Box<dyn Effect>) {
        panic!("No add child functionality for this effect.");
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas;
}

#[derive(Debug)]
pub struct Add {
    pub children: Vec<Box<dyn Effect>>,
}
impl Effect for Add {
    fn add_child(&mut self, effect: Box<dyn Effect>) {
        self.children.push(effect);
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas {
        // first, retrieve all child updates.
        let mut child_states = self
            .children
            .iter_mut()
            .map(|x| x.update(state))
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

#[derive(Debug)]
pub struct Sub {
    pub children: Vec<Box<dyn Effect>>,
}
impl Effect for Sub {
    fn add_child(&mut self, effect: Box<dyn Effect>) {
        self.children.push(effect);
    }

    fn update(&mut self, state: &mut dyn State) -> Canvas {
        // first, retrieve all child updates.
        let mut child_states = self
            .children
            .iter_mut()
            .map(|x| x.update(state))
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Store {
    pub name: String,
    pub child: Option<Box<dyn Effect>>,
}
impl Effect for Store {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let canvas = self.child.as_mut().unwrap().update(state);
        state.set_stored(&self.name, canvas.clone());
        canvas
    }
    fn add_child(&mut self, effect: Box<dyn Effect>) {
        self.child = Some(effect);
    }
}

#[derive(Debug)]
pub struct Static {
    pub color: RGBA,
}
impl Effect for Static {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut canvas = state.get_canvas();
        for p in canvas.iter_mut() {
            *p = self.color;
        }
        canvas
    }
}

#[derive(Debug)]
pub struct SetAlpha {
    pub child: Option<Box<dyn Effect>>,
    pub value: f64,
}
impl Effect for SetAlpha {
    fn update(&mut self, state: &mut dyn State) -> Canvas {
        let mut canvas = self.child.as_mut().unwrap().update(state);
        for p in canvas.iter_mut() {
            p.set_alpha(self.value);
        }
        canvas
    }
    fn add_child(&mut self, effect: Box<dyn Effect>) {
        self.child = Some(effect);
    }
}
