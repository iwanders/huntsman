use crate::effects::{make_effect, EffectPtr};

use crate::effects::{Add, Retrieve, Store, Sub};
use crate::effects::{MovingKernel, MovingParticles, Rectangle};
use crate::effects::{SetAlpha, Colorize, Static};

use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Debug)]
struct LoaderError {
    details: String,
}

impl LoaderError {
    fn new(msg: &str) -> LoaderError {
        LoaderError {
            details: msg.to_string(),
        }
    }
    fn boxed(msg: String) -> Box<LoaderError> {
        Box::new(LoaderError::new(msg.as_str()))
    }
}
impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "LoaderError {}", self.details)
    }
}
impl std::error::Error for LoaderError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EffectConfig {
    Add,
    Sub,
    SetAlpha(SetAlpha),
    Static(Static),
    Store(Store),
    Retrieve(Retrieve),
    Rectangle(Rectangle),
    MovingKernel(MovingKernel),
    MovingParticles(MovingParticles),
    Colorize(Colorize),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EffectSpecification {
    // effect: String,
    name: String,

    #[serde(flatten)]
    config: EffectConfig,
    children: Option<Vec<String>>,

    #[serde(default)]
    root: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EffectStorage {
    pub effects: Vec<EffectSpecification>,
    // more files to load? who knows.
}

pub fn load_effects(filename: &str) -> Result<EffectStorage, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename).expect("file should be opened");
    if filename.ends_with("json") {
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");
        let effects: Vec<EffectSpecification> = serde_json::from_value(
            json.get("effects")
                .expect("file should have effects key")
                .clone(),
        )?;
        return Ok(EffectStorage { effects });
    }
    if filename.ends_with("yaml") {
        let yaml: serde_yaml::Value =
            serde_yaml::from_reader(file).expect("file should be proper yaml");
        let effects: Vec<EffectSpecification> = serde_yaml::from_value(
            yaml.get("effects")
                .expect("file should have effects key")
                .clone(),
        )?;
        return Ok(EffectStorage { effects });
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Format not understood",
    )))
}

pub fn make_effects_simple(
    specs: &[EffectSpecification],
) -> Result<Vec<EffectPtr>, Box<dyn std::error::Error>> {
    // need two passes, first to set up the elements
    // second to connect all the childs
    // then, convert it back to the vector we need, holding only the root elements.
    let mut effects: Vec<EffectPtr> = Vec::new();
    let mut effects_map: std::collections::HashMap<String, EffectPtr> = Default::default();

    // First pass, create our effects
    for spec in specs.iter() {
        let new_effect: EffectPtr;
        match &spec.config {
            EffectConfig::Add => new_effect = Add::new(),
            EffectConfig::Sub => new_effect = Sub::new(),
            EffectConfig::SetAlpha(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::Store(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::Retrieve(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::Static(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::Rectangle(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::MovingKernel(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::MovingParticles(v) => {
                new_effect = make_effect(v.clone());
            }
            EffectConfig::Colorize(v) => {
                new_effect = make_effect(v.clone());
            }
        }

        let had_old = effects_map.insert(spec.name.clone(), new_effect);
        if !had_old.is_none() {
            return Err(Box::new(LoaderError::new(&format!(
                "Effect named {} was present twice",
                spec.name
            ))));
        }
    }

    // That was the first pass that created all the elements, now we do the second pass to connect them all.
    for spec in specs.iter() {
        let our_effect = effects_map.get(&spec.name).unwrap(); // must be present from above loop.
        if let Some(children) = &spec.children {
            for child in children.iter() {
                // add all the childs to our effect.
                // look up our child in the list of effects.
                let child_effect = effects_map.get(child);
                if child_effect.is_none() {
                    return Err(LoaderError::boxed(format!(
                        "Child named {} was not present",
                        child
                    )));
                }
                let child_effect = child_effect.unwrap();
                our_effect.borrow_mut().add_child(Rc::clone(child_effect));
            }
        }
        if spec.root {
            effects.push(Rc::clone(our_effect))
        }
    }

    Ok(effects)
}
