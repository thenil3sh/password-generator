#![windows_subsystem = "windows"]
use slint::SharedString;
use std::{cell::RefCell, rc::Rc};
mod func;
mod pass;
use clipboard::{ClipboardContext, ClipboardProvider};
use func::{charray, warray};
use pass::{pass_phrase, placeholder, Include};
slint::include_modules!();

#[derive(Debug, Clone, Copy)]
enum Mode {
    Random,
    Passphrase,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    println!("oreo");

    let window = AppWindow::new().unwrap();

    let length: Rc<RefCell<u8>> = Rc::new(RefCell::new(6));
    let symbols = Rc::new(RefCell::new(false));
    let nums = Rc::new(RefCell::new(false));
    let caps = Rc::new(RefCell::new(false));
    let word = Rc::new(RefCell::new(false));
    let mode = Rc::new(RefCell::new(Mode::Random));
    let base_pass = Rc::new(RefCell::new(String::new()));
    let choose: Rc<RefCell<Vec<Include>>> = Rc::new(RefCell::new(Vec::new()));

    let len_slide = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);
    let mode_clone = Rc::clone(&mode);
    let base_clone = Rc::clone(&base_pass);

    window.on_len_mod(move |length| {
        let app = len_slide.upgrade().unwrap();
        let mode = mode_clone.borrow();
        let mut base_pass = base_clone.borrow_mut();
        match *mode {
            Mode::Random => {
                if base_pass.is_empty() {
                    app.set_text_opacity(0.9);
                }
                let mut len_val = len_clone.borrow_mut();
                *len_val = length as u8;

                *base_pass = if *word_clone.borrow() {
                    warray(
                        *len_val,
                        *caps_clone.borrow(),
                        *num_clone.borrow(),
                        *sym_clone.borrow(),
                        '-',
                    )
                } else {
                    charray(
                        *len_val,
                        *caps_clone.borrow(),
                        *num_clone.borrow(),
                        *sym_clone.borrow(),
                    )
                };
            }
            Mode::Passphrase => {
                panic!("Ok what? PANIC BY LENGTH MODIFICATION");
            }
        }
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(SharedString::from(&*base_pass.clone()));
    });

    let edited_text = window.as_weak();

    let mode_select = window.as_weak();
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let words_clone = Rc::clone(&word);
    let base_clone = Rc::clone(&base_pass);
    let mode_clone = Rc::clone(&mode);
    let len_clone = Rc::clone(&length);
    window.on_mode_select(move |random| {
        print!("mode changed! ");
        let app = mode_select.upgrade().unwrap();
        let mut mode = mode_clone.borrow_mut();
        let mut base_pass = base_clone.borrow_mut();

        *mode = if random {
            println!("We're at random!!");
            app.set_result(SharedString::from(if *words_clone.borrow() {
                warray(
                    *len_clone.borrow(),
                    *caps_clone.borrow(),
                    *num_clone.borrow(),
                    *sym_clone.borrow(),
                    '-',
                )
            } else {
                charray(
                    *len_clone.borrow(),
                    *caps_clone.borrow(),
                    *num_clone.borrow(),
                    *sym_clone.borrow(),
                )
            }));
            Mode::Random
        } else {
            println!("Naw we're at passPhrase");
            base_pass.clear();
            app.set_result(SharedString::from("Type in"));
            Mode::Passphrase
        }
    });

    // Clone references for the symbol_toggle closure
    let sym_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);
    let mode_clone = Rc::clone(&mode);
    let base_clone = Rc::clone(&base_pass);
    let choose_clone = Rc::clone(&choose);
    window.on_symtoggled(move || {
        let app = sym_toggle.upgrade().unwrap();
        // Mutate the symbols flag through the RefCell
        let mut sym_val = sym_clone.borrow_mut();
        *sym_val = !*sym_val; // Toggle the value
        let mode = mode_clone.borrow();
        let mut base_pass = base_clone.borrow_mut();
        match *mode {
            Mode::Random => {
                *base_pass = if *word_clone.borrow() {
                    warray(
                        *len_clone.borrow(),
                        *caps_clone.borrow(),
                        *num_clone.borrow(),
                        *sym_val,
                        '-',
                    )
                } else {
                    charray(
                        *len_clone.borrow(),
                        *caps_clone.borrow(),
                        *num_clone.borrow(),
                        *sym_val,
                    )
                };
                app.set_result(SharedString::from(&*base_pass));
            }
            Mode::Passphrase => {
                if base_pass.is_empty() {
                    println!("Base pass is empty");
                    return ();
                }
                let mut choose = choose_clone.borrow_mut();
                let mut oreo = String::new();
                choose.clear();
                if base_pass.is_empty() {
                    app.set_result(SharedString::from(placeholder()));
                    app.set_text_opacity(0.4);
                } else {
                    for i in base_pass.chars() {
                        oreo.push(pass_phrase(
                            i,
                            *sym_val,
                            *num_clone.borrow(),
                            '_',
                            &mut choose,
                            3,
                        ));
                    }
                    *base_pass = oreo;
                    app.set_result(SharedString::from(&*base_pass));
                }
            }
        }
        if !base_pass.is_empty() {
            app.set_text_opacity(0.9);
        }
        println!("\nSymbols: {}", *sym_val);
        app.set_copy_state(SharedString::from("Copy Password"));
    });

    let number_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);
    let mode_clone = Rc::clone(&mode);
    let base_clone = Rc::clone(&base_pass);
    let choose_clone = Rc::clone(&choose);

    window.on_numtoggled(move || {
        let app = number_toggle.upgrade().unwrap();
        let mut nums_value = num_clone.borrow_mut();
        *nums_value = !*nums_value; // Toggle the value
        let mut base_pass = base_clone.borrow_mut();
        let mode = mode_clone.borrow();

        match *mode {
            Mode::Random => {
                *base_pass = if *word_clone.borrow() {
                    warray(
                        *len_clone.borrow(),
                        *caps_clone.borrow(),
                        *nums_value,
                        *sym_clone.borrow(),
                        '-',
                    )
                } else {
                    charray(
                        *len_clone.borrow(),
                        *caps_clone.borrow(),
                        *nums_value,
                        *sym_clone.borrow(),
                    )
                };
            }
            Mode::Passphrase => {
                let mut choose = choose_clone.borrow_mut();
                let mut oreo = String::new();
                choose.clear();
                for i in base_pass.chars() {
                    oreo.push(pass_phrase(
                        i,
                        *sym_clone.borrow(),
                        *nums_value,
                        '_',
                        &mut choose,
                        3,
                    ));
                }
                *base_pass = oreo;
            }
        }
        if !base_pass.is_empty() {
            app.set_text_opacity(0.9);
        }
        println!("\nNumbers: {}", *nums_value);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(SharedString::from(&*base_pass));
    });

    let capital_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let cap_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);
    let mode_clone = Rc::clone(&mode);
    let base_clone = Rc::clone(&base_pass);

    window.on_captoggled(move || {
        let app = capital_toggle.upgrade().unwrap();

        let mut caps_val = cap_clone.borrow_mut();
        *caps_val = !*caps_val;
        let mut base_pass = base_clone.borrow_mut();
        let mode = mode_clone.borrow_mut();

        match *mode {
            Mode::Random => {
                *base_pass = if *word_clone.borrow() {
                    warray(
                        *len_clone.borrow(),
                        *caps_val,
                        *num_clone.borrow(),
                        *sym_clone.borrow(),
                        '-',
                    )
                } else {
                    charray(
                        *len_clone.borrow(),
                        *caps_val,
                        *num_clone.borrow(),
                        *sym_clone.borrow(),
                    )
                };
            }
            Mode::Passphrase => {}
        }
        if !base_pass.is_empty() {
            app.set_text_opacity(0.9);
        }
        println!("\nCapitls : {}", *caps_val);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(SharedString::from(&*base_pass));
    });

    let word_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let cap_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);
    let mode_clone = Rc::clone(&mode);
    let base_clone = Rc::clone(&base_pass);

    window.on_wortoggled(move || {
        let app = word_toggle.upgrade().unwrap();
        let mut wor_val = word_clone.borrow_mut();
        *wor_val = !*wor_val;
        let mut base_pass = base_clone.borrow_mut();
        let mode = mode_clone.borrow();

        match *mode {
            Mode::Random => {
                *base_pass = if *wor_val {
                    warray(
                        *len_clone.borrow(),
                        *cap_clone.borrow(),
                        *num_clone.borrow(),
                        *sym_clone.borrow(),
                        '-',
                    )
                } else {
                    charray(
                        *len_clone.borrow(),
                        *cap_clone.borrow(),
                        *num_clone.borrow(),
                        *sym_clone.borrow(),
                    )
                };
                app.set_symbol_state(!app.get_symbol_state());
            }
            Mode::Passphrase => {
                panic!("No way!??? whaat");
            }
        }
        if !base_pass.is_empty() {
            app.set_text_opacity(0.9);
        }
        println!("\nWord : {}", *wor_val);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(SharedString::from(&*base_pass));
    });

    let copy_btn = window.as_weak();
    let base_clone = Rc::clone(&base_pass);
    window.on_copy_clicked(move || {
        let app = copy_btn.upgrade().unwrap();
        app.set_copy_state(SharedString::from("Copied!"));

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(base_clone.borrow().to_owned()).unwrap();
    });

    let edited_text = window.as_weak();
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let choose_clone = Rc::clone(&choose);
    let base_clone = Rc::clone(&base_pass);
    window.on_edited_text(move |oreo| {
        let mut choose = choose_clone.borrow_mut();
        let app = edited_text.upgrade().unwrap();
        let oreo = oreo.to_string();
        let mut base_pass = base_clone.borrow_mut();
        if oreo.is_empty() {
            choose.clear();
            base_pass.clear();
            println!("oreo is empty?");
            let placeholder = placeholder();
            app.set_result(SharedString::from(&placeholder));
            app.set_text_opacity(0.4);
            return ();
        }
        while oreo.len() <= base_pass.len() {
            base_pass.pop();
        }
        if oreo.len() > base_pass.len() {
            base_pass.push(pass_phrase(
                match oreo.chars().rev().nth(0) {
                    Some(x) => x,
                    None => {
                        println!("Breaks!!");
                        ' '
                    }
                },
                *sym_clone.borrow(),
                *num_clone.borrow(),
                '.',
                &mut choose,
                2,
            ));
        }
        if app.get_text_opacity() < 1.0 {
            app.set_text_opacity(1.0);
        }
        app.set_result(SharedString::from(&*base_pass));
    });

    window.run().unwrap();
}
