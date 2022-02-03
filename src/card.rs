use crate::card::Class::Undefined;

#[derive(Eq, PartialEq)]
pub enum Class {
    Defined(str),
    Undefined,
}

#[derive(Eq, PartialEq)]
pub enum Expression {
    Defined(str),
    Undefined,
}

#[derive(Eq, PartialEq)]
pub struct Card {
    class: Class,
    expression: Expression,
}

impl Card {
    pub fn new() -> Card {
        Card {
            class: Class::Undefined,
            expression: Expression::Undefined,
        }
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = class;
    }

    pub fn set_expression(&mut self, expression: Expression) {
        self.expression = expression;
    }
}
