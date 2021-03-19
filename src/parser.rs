pub struct DirectiveParser<'a> {
    pub text: &'a str,
    pub position: usize,
}

impl<'a> DirectiveParser<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            position: 0,
        }
    }

    pub fn current_char(&self) -> char {
        self.text[self.position..=self.position+1].chars().nth(0).unwrap()
    }

    pub fn previous_char(&self) -> char {
        self.text[self.position-1..=self.position].chars().nth(0).unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.current_char().is_whitespace() {
            self.advance();
        }
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn expect(&mut self, token: char) -> bool {
        self.skip_whitespace();

        if self.current_char() == token {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn expect_sequence(&mut self, sequence: &[char]) -> bool {
        for item in sequence {
            if self.expect(*item) {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    pub fn parse_word(&mut self, word: &str) -> bool {
        if word.contains(" ") {
            // TODO: Ensure `word` does not have whitespace
            panic!("Whitespace in word");
        }

        self.skip_whitespace();

        for character in word.chars() {
            if self.current_char() == character {
                self.advance();
            } else {
                return false;
            }
        }

        true
    }

    pub fn parse_next_word(&mut self) -> &str {
        self.skip_whitespace();

        let start = self.position;

        loop {
            // Starts with letter or underscore
            if (self.position - start == 0) && !(self.current_char().is_alphabetic() || self.current_char() == '_') {
                break;
            } else if self.current_char().is_alphanumeric() || self.current_char() == '_' {
                self.advance();
            } else {
                break;
            }
        }

        &self.text[start..self.position]
    }

    pub fn parse_previous_word(&mut self) -> &str {
        let original = self.position;

        // TODO: Ensure this is correct
        self.position -= 1;

        while self.current_char().is_whitespace() {
            self.position -= 1;
        }

        let end = self.position;

        loop {
            if self.current_char().is_alphanumeric() || self.current_char() == '_' {
                self.position -= 1;
            } else {
                break;
            }
        }
        // TODO: Ensure word doesn't start with numbers -> remove numbers if it does

        let start = self.position;
        self.position = original;

        // FIXME: Why off by one?
        &self.text[start+1..end+1]
    }

    pub fn parse_until(&mut self, stop: char) -> &str {
        self.skip_whitespace();
        let start = self.position;

        while self.current_char() != stop {
            self.advance();
        }

        &self.text[start..self.position]
    }

    pub fn skip_to(&mut self, item: &str) {
        if let Some(location) = self.text[self.position..].find(item) {
            self.position += location;
        } else {
            todo!("not found")
        }
    }
}