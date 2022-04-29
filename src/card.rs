use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Class<'a> {
    Defined(&'a str),
    Undefined,
}

impl<'a> Display for Class<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Defined(class) => write!(class),
            Class::Undefined => write!("Undefined"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expression<'a> {
    Defined(&'a str),
    Undefined,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Card<'a> {
    class: Class<'a>,
    expression: Expression<'a>,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        Card {
            class: Class::Undefined,
            expression: Expression::Undefined,
        }
    }

    pub fn new_defined(class: &'a str, expression: &'a str) -> Card<'a> {
        Card {
            class: Class::Defined(class),
            expression: Expression::Defined(expression),
        }
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn set_class(&mut self, class: Class<'a>) {
        self.class = class;
    }

    pub fn set_expression(&mut self, expression: Expression<'a>) {
        self.expression = expression;
    }

    pub fn copy_card_values(&mut self, card: &'a Card) {
        self.set_class(card.class.clone());
        self.set_expression(card.expression.clone());
    }
}
