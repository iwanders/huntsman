use crate::effects::{Add, Effect, HorizontalMovingPixel, Retrieve, SetAlpha, SetAlphaConfig, Static, Store, Sub};



#[derive(Debug)]
struct LoaderError {
    details: String
}

impl LoaderError {
    fn new(msg: &str) -> LoaderError {
        LoaderError{details: msg.to_string()}
    }
    fn boxed(msg: String) -> Box<LoaderError>
    {
        Box::new(LoaderError::new(msg.as_str()))
    }
}
impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"LoaderError {}",self.details)
    }
}
impl std::error::Error for LoaderError {
    fn description(&self) -> &str {
        &self.details
    }
}


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum EffectConfig
{
    None,
    SetAlpha(SetAlphaConfig),
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EffectSpecification {
    effect: String,
    name: String,
    config: EffectConfig,
    children: Vec<String>,

    #[serde(default)]
    root: bool,
}



pub fn z()
{
    let t = EffectSpecification{effect: "Add".to_string(), name: "add_thing".to_string(), config:EffectConfig::SetAlpha(SetAlphaConfig{value: 0.5}), children: vec!("foo".to_string(), "bar".to_string()), root: false};
    let serialized = serde_json::to_string(&t).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: EffectSpecification = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EffectStorage
{
    pub effects: Vec<EffectSpecification>,
    // more files to load? who knows.
}

pub fn load_effects(filename: &str) -> Result<EffectStorage, Box<dyn std::error::Error>>
{
    let file = std::fs::File::open(filename).expect("file should be opened");
    if filename.ends_with("json")
    {
        let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
        let effects: Vec<EffectSpecification> = serde_json::from_value(json.get("effects").expect("file should have effects key").clone())?;
        return Ok(EffectStorage{effects});
    }
    if filename.ends_with("yaml")
    {
        let yaml: serde_yaml::Value = serde_yaml::from_reader(file).expect("file should be proper yaml");
        let effects: Vec<EffectSpecification> = serde_yaml::from_value(yaml.get("effects").expect("file should have effects key").clone())?;
        return Ok(EffectStorage{effects});
    }
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Format not understood")))
}

pub fn make_effects_simple(specs: &[EffectSpecification]) -> Result<Vec<Box<dyn Effect>>, Box<dyn std::error::Error>>
{

    // need two passes, first to set up the elements
    // second to connect all the childs
    // then, convert it back to the vector we need, holding only the root elements.
    let mut effects: Vec<Box<dyn Effect>> = Vec::new();
    let mut effects_map: std::collections::HashMap<String, Box<dyn Effect>> = Default::default();

    for spec in specs.iter()
    {
        let new_effect : Box<dyn Effect>;
        match spec.effect.as_str()
        {
            "Add" => {
                new_effect = Box::new(Add { children: vec![] });
            },
            _ => {return Err(Box::new(LoaderError::new(&format!("Effect {} not supported", spec.effect))))},
        }
        
        let had_old = effects_map.insert(spec.name.clone(), new_effect);
        if !had_old.is_none()
        {
            return Err(Box::new(LoaderError::new(&format!("Effect named {} was present twice", spec.name))));
        }
    }

    // That was the first pass that created all the elements, now we do the second pass to connect them all.
    for i in 0..100
    {
        for spec in specs.iter()
        {
            let our_effect = effects_map.get(&spec.name).unwrap();  // must be present from above loop.
            for child in spec.children.iter()
            {
                
            }
        }
    }
    

    Ok(effects)
}


