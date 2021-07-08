#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Usage {
    Selector,
    DynamicFlag,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Key {
    pub name: &'static str,
    pub hid: usize,
    pub at101: Option<usize>,
    pub desc: &'static str,
    pub usage: Usage,
}
