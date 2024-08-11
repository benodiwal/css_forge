#[derive(Debug, PartialEq, Clone)]
pub struct StyleSheet {
    pub rules: Vec<Rule>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Selector {
    Simple(SimpleSelector)
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Px,
    Em,
    Rem
}

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
