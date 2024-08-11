use crate::{css::{Declaration, Rule, Selector, SimpleSelector, StyleSheet}, errors::CssParseError};

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
        let declarations = Vec::new();
        Ok(declarations)
    }

    fn consume_declaration(&mut self) -> Result<Declaration, CssParseError> {
        todo!()
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
