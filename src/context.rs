use std::collections::HashMap;

use crate::parser::DirectiveParser;


pub trait GenerateDirectives {
    fn handle_custom_directive(&mut self, directive: &str, parser: &mut DirectiveParser) -> String;
}

pub struct Context<State : GenerateDirectives> {
    pub state: State,
    pub key: char,
    pub directives: HashMap<String, Box<dyn FnMut(&mut DirectiveParser, &mut State) -> String>>,
}

impl<State : GenerateDirectives> Context<State> {
    pub fn new(state: State) -> Self {
        Self {
            state,
            key: '\0',
            directives: HashMap::new(),
        }
    }

    pub fn with_key(mut self, key: char) -> Self {
        self.key = key;
        self
    }

    pub fn with_directive(mut self, directive: &str, func: impl FnMut(&mut DirectiveParser, &mut State) -> String + 'static) -> Self {
        if let Some(_existing) = self.directives.insert(directive.into(), Box::new(func)) {
            panic!("Directive `{}` already exists", directive);
        }
        self
    }

    pub fn insert_directive(&mut self, directive: String, func: impl FnMut(&mut DirectiveParser, &mut State) -> String + 'static) {
        // FIXME: Clone is only needed because of the panic
        if let Some(_existing) = self.directives.insert(directive.clone(), Box::new(func)) {
            panic!("Directive `{}` already exists", directive);
        }
    }
}