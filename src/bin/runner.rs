use meta_extension::prelude::*;

use std::path::{Path, PathBuf};

struct State {
    /// Type Name -> Ordered Fields
    constructors: std::collections::HashMap<String, Vec<String>>,
}

impl State {
    fn new() -> Self {
        Self {
            constructors: std::collections::HashMap::new(),
        }
    }
}

impl GenerateDirectives for State {
    fn handle_custom_directive(&mut self, directive: &str, parser: &mut DirectiveParser) -> String {    
        // TODO: Output
        let mut output = String::new();
        
        if let Some(constructor) = self.constructors.get(directive) {
            output.push_str(&format!("{} {{\n", directive));

            let mut fields = std::collections::HashMap::new();

            parser.expect('{');
            
            // TODO: Do not require trailing comma
            while parser.expect('.') {
                let field = parser.parse_next_word().to_owned();
                parser.expect('=');
                let value = parser.parse_until(',').to_owned();
                parser.advance();

                // TEMP: `clone` needed to print after
                fields.insert(field.clone(), value.clone());

                println!("\tASSIGNED: `{}: {}`", field, value);

                if parser.expect('}') {
                    break;
                }
            }
            // TODO: Build valid constructor (field names, order, etc.)
            if constructor.len() != fields.len() {
                // TODO:
                todo!("Error on different lengths");
            }

            for field in constructor {
                if let Some(value) = fields.get(field) {
                    output.push_str(&format!("    {},\n", value));
                } else {
                    // TODO:
                    todo!("Error on field not found");
                }
            }
            // Remove trailing ",\n"
            output.pop(); output.pop();
            output.push_str("\n}");
            output.push_str(parser.remaining());
        } else {
            // TODO:
            todo!("Error on directive not found")
        }

        output
    }
}

fn main() {
    /*** NOTE: The following section would be obtained from an external definition ***/

    // TODO: Get context externally
    let mut context = Context::new(State::new())
        .with_key('@')
        .with_directive("register", |parser, state| {
            if parser.parse_word("struct") {
                let name = parser.parse_next_word().to_owned();
                println!("\tNAME: `{}`", name);
                
                let mut fields = Vec::new();

                loop {
                    parser.skip_to(";");

                    // end of struct
                    if parser.previous_char() == '}' {
                        break;
                    }

                    let field = parser.parse_previous_word();
                    println!("\tFIELD: `{}`", field);
                    fields.push(field.to_owned());
                    parser.advance();
                }

                state.constructors.insert(name, fields);

            }
            String::from(parser.text)
        });

    // TODO: Get target directory externally
    let mex_dir = "./tests";

    /*** End external section ***/
    
    let targets = get_mex_files(mex_dir).unwrap();

    
    for target in targets {
        let mut output = String::new();
        println!("TARGET: {:?}\n", target);
        let source = std::fs::read_to_string(&target).unwrap();

        // Find each key usage
        for (i, group) in source.split(context.key).enumerate() {
            // Skip the first group (everything prior to the first key)
            if i == 0 {
                output.push_str(group);
                continue;
            }

            // Get the directive
            let mut directive_end = 0;
            for (i, character) in group.char_indices() {
                if !character.is_alphanumeric() && character != '_'{
                    directive_end = i;
                    break;
                }
            };

            let directive = &group[0..directive_end];
            let body = &group[directive_end..];

            println!("DIRECTIVE: {}", directive);
            // println!("PARSING:\n{}\n", body);
            let mut parser = meta_extension::parser::DirectiveParser::new(body);

            if let Some(function) = context.directives.get_mut(directive) {
                output.push_str(&(function)(&mut parser, &mut context.state));
            } else {
                // TODO: Error -> no handler for the directive
                output.push_str(&context.state.handle_custom_directive(directive, &mut parser));
            }

        }
        println!("\n---{:?}---\nOUTPUT:\n{}", target, output);

        // Write generated file
        let output_path = target.to_string_lossy().replace(".mex", "");
        std::fs::write(output_path, output).unwrap();
    }
}

fn get_mex_files(target: &str) -> std::io::Result<Vec<PathBuf>> {
    fn append_mex_files<P: AsRef<Path> + std::fmt::Debug>(target: P, targets: &mut Vec<PathBuf>) -> std::io::Result<()>{
        for entry in std::fs::read_dir(target)? {
            let path = entry?.path();
            if path.is_dir() {
                append_mex_files(path, targets)?;
            } 
            else if path.to_string_lossy().contains(".mex.") {
                targets.push(path);
            }
        }
        
        Ok(())
    }

    let mut targets = Vec::new();

    append_mex_files(target, &mut targets)?;
    
    Ok(targets)
}