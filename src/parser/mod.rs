use crate::{css::{Color, Declaration, Rule, Selector, SimpleSelector, StyleSheet, Unit, Value}, errors::CssParseError};

pub struct Parser {
    input: String,
    poisition: usize
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser {
            input,
            poisition: 0
        }
    }

    pub fn parse(&mut self) -> Result<StyleSheet, CssParseError> {
        let mut stylesheet = StyleSheet { rules: Vec::new() };
        while !self.eof() {
            self.consume_whitespace();
            if self.eof() { break; }
            let rule = self.parse_rule()?;
            stylesheet.rules.push(rule);
        }
        Ok(stylesheet)
    }

    pub fn parse_rule(&mut self) -> Result<Rule, CssParseError> {
        let selectors = self.parse_selectors()?;
        self.consume_whitespace();
        self.consume_char('{')?;

        let declarations = self.parse_declarations()?;
        self.consume_char('}')?;

        Ok(Rule {
            selectors,
            declarations
        })
    }

    fn parse_selectors(&mut self) -> Result<Vec<Selector>, CssParseError> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()?));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char(',')?;
                    self.consume_whitespace();
                },
                '{' => break,
                _ => return Err(CssParseError::InvalidSelector)
            }
        }

        Ok(selectors)
    }

    fn parse_simple_selector(&mut self) -> Result<SimpleSelector, CssParseError> {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new()
        };
        
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char('#')?;
                    selector.id = Some(self.parse_identifier()?);
                },
                '.' => {
                    self.consume_char('.')?;
                    selector.class.push(self.parse_identifier()?);
                }
                '*' => {
                    self.consume_char('*')?;
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier()?);
                }
                _ => break,
            }
        }

        Ok(selector)
    }

    fn parse_identifier(&mut self) -> Result<String, CssParseError> {
        let identifer = self.consume_while(valid_identifier_char);
        if identifer.is_empty() {
            Err(CssParseError::InvalidValue)
        } else {
            Ok(identifer)
        }
    }

    fn parse_declarations(&mut self) -> Result<Vec<Declaration>, CssParseError> {
        let mut declarations = Vec::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '}' { break; }
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
            self.consume_whitespace();
            self.consume_char(';')?;
        }

        Ok(declarations)
    }

    fn parse_declaration(&mut self) -> Result<Declaration, CssParseError> {
        let property_name = self.parse_identifier()?;
        self.consume_whitespace();
        self.consume_char(':')?;
        self.consume_whitespace();
        let value = self.parse_value()?;

        Ok(Declaration {
            name: property_name,
            value
        })
    }

    fn parse_value(&mut self) -> Result<Value, CssParseError> {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Ok(Value::Keyword(self.parse_identifier()?))
        }
    }

    fn parse_length(&mut self) -> Result<Value, CssParseError> {
        let num_str = self.consume_while(|c| c.is_ascii_digit() || c == '.');
        let num = num_str.parse::<f32>().map_err(|_| CssParseError::InvalidValue)?;
        let unit = self.parse_unit()?;
        Ok(Value::Length(num, unit))
    }

    fn parse_unit(&mut self) -> Result<Unit, CssParseError> {
        let unit_str = self.parse_identifier()?;
        match &unit_str.to_lowercase()[..] {
            "px" => Ok(Unit::Px),
            "em" => Ok(Unit::Em),
            "rem" => Ok(Unit::Rem),
            _ => Err(CssParseError::InvalidUnit)
        }
    }

    fn parse_color(&mut self) -> Result<Value, CssParseError> {
        self.consume_char('#')?;
        let color_str = self.consume_while(|c| c.is_ascii_hexdigit());
        if color_str.len() != 6 {
            return Err(CssParseError::InvalidColor);
        }
        let r = u8::from_str_radix(&color_str[0..2], 16).map_err(|_| CssParseError::InvalidColor)?;
        let g = u8::from_str_radix(&color_str[2..4], 16).map_err(|_| CssParseError::InvalidColor)?;
        let b = u8::from_str_radix(&color_str[4..6], 16).map_err(|_| CssParseError::InvalidColor)?;

        Ok(Value::ColorValue(Color { r, g, b, a: 255 }))
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
    F: Fn(char) -> bool
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char(' ').unwrap());
        }
        result
    }

    fn consume_char(&mut self, expected: char) -> Result<char, CssParseError> {
        if self.eof() {
            return Err(CssParseError::UnexpectedEOF);
        }
        let next_char = self.input[self.poisition..].chars().next().unwrap();
        if next_char == expected {
            self.poisition += next_char.len_utf8();
            Ok(next_char)
        } else {
            Err(CssParseError::UnexpectedToken(next_char))
        }
    }

    fn next_char(&self) -> char {
        self.input[self.poisition..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.poisition >= self.input.len()
    }

}

fn valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}
