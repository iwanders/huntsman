#[derive(Debug, Clone, Copy)]
pub enum Usage {
    Selector,
    DynamicFlag,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub hid: usize,
    pub at101: Option<usize>,
    pub desc: &'static str,
    pub usage: Usage,
}
