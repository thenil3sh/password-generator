use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Include {
    Characters,
    Numbers,
    Symbols,
}

pub enum Placeholder {
    Formal,
    Easter,
}

#[derive(Clone, Copy)]
pub enum Delimeter {
    Period,
    Hyphen,
    Underscore,
    Commas,
    Space,
    Random,
}

pub fn pass_phrase(
    last: char,
    symbols: bool,
    numbers: bool,
    delimeter: char,
    choose: &mut Vec<Include>,
    crazy_lvl: u8,
) -> char {
    if choose.is_empty() {
        if numbers { choose.push(Include::Numbers);}
        if symbols { choose.push(Include::Symbols);}
        choose.push(Include::Characters);
    }
    let num_collection = if numbers {
        choose.push(Include::Numbers);
        HashMap::from([
            ('A', '4'),
            ('a', '4'),
            ('I', '1'),
            ('i', '1'),
            ('L', 'l'),
            ('l', '1'),
            ('o', '0'),
            ('O', '0'),
            ('S', '5'),
            ('s', '5'),
            ('T', '7'),
            ('Z', '5'),
        ])
    } else {
        HashMap::new()
    };

    let sym_collection = if symbols {
        choose.push(Include::Symbols);
        HashMap::from([
            ('a', '@'),
            ('c', '<'),
            ('C', '('),
            ('I', '!'),
            ('i', ':'),
            ('s', '$'),
            ('S', '$'),
            ('o', 'O'),
        ])
    } else {
        HashMap::new()
    };

    let char = if last == ' ' {
        delimeter
    } else {
        match choose.choose(&mut thread_rng()).unwrap() {
            Include::Numbers => match num_collection.get(&last) {
                Some(x) => {
                    if symbols {
                        choose.push(Include::Symbols);
                    }
                    if crazy_lvl == 1 {
                        *[*x, last].choose(&mut thread_rng()).unwrap()
                    } else {
                        *x
                    }
                }
                None => {
                    choose.push(Include::Symbols);
                    last
                }
            },
            Include::Symbols => match sym_collection.get(&last) {
                Some(x) => {
                    if numbers {
                        choose.push(Include::Numbers);
                    }
                    *x
                }
                None => {
                    choose.push(Include::Numbers);
                    last
                }
            },
            _ => last,
        }
    };
    char
}

pub fn placeholder() -> String {                 ////////////// // doesnt leave empty plaeholder alone :c
    let string = [Placeholder::Formal, Placeholder::Formal, Placeholder::Formal, Placeholder::Formal, Placeholder::Easter]
        .choose(&mut thread_rng())
        .unwrap();
    let string = match string {

        Placeholder::Formal => *["Enter passphrase"].choose(&mut thread_rng()).unwrap(),
        Placeholder::Easter => *[
            "/* empty */",
            "* empty *",
            "* empty again *",
            "swoosh~!",
            "poof! gone",
            "(⁠｡⁠•̀⁠ᴗ⁠-⁠)⁠✧",
            "(O_O;)",
            "no Text?",
        ]
        .choose(&mut thread_rng())
        .unwrap(),
    };
    String::from(string)
}