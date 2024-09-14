use slint::SharedString;
use std::{cell::RefCell, rc::Rc};
mod func;
use clipboard::{ClipboardContext, ClipboardProvider};
use func::{charray, warray};
slint::include_modules!();

fn main() {
    println!("oreo");
 
    let window = AppWindow::new().unwrap();

    // Wrap booleans in Rc<RefCell> to allow shared mutable access
    let length : Rc<RefCell<u8>> = Rc::new(RefCell::new(6));
    let symbols = Rc::new(RefCell::new(false));
    let nums = Rc::new(RefCell::new(false));
    let caps = Rc::new(RefCell::new(false));
    let word = Rc::new(RefCell::new(false));

    // Clones for Slider : 
    let len_slide = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);

    window.on_len_mod(move |length| {
        let app = len_slide.upgrade().unwrap();

        let mut len_val = len_clone.borrow_mut();
        *len_val = length as u8;

        let string = if *word_clone.borrow() {
            warray(*len_val, *caps_clone.borrow(), *num_clone.borrow(), *sym_clone.borrow(), '-')
        } else {
            charray(*len_val, *caps_clone.borrow(), *num_clone.borrow(), *sym_clone.borrow())
        };

        let string = SharedString::from(string);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(string);
        
    });


    
    // Clone references for the symbol_toggle closure
    let sym_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);



    window.on_symtoggled(move || {
        let app = sym_toggle.upgrade().unwrap();

        // Mutate the symbols flag through the RefCell
        let mut sym_val = sym_clone.borrow_mut();
        *sym_val = !*sym_val; // Toggle the value

        // Use the updated values
        let string = if *word_clone.borrow() {
            warray(*len_clone.borrow(), *caps_clone.borrow(), *num_clone.borrow(), *sym_val, '-')
        } else {
            charray(*len_clone.borrow(), *caps_clone.borrow(), *num_clone.borrow(), *sym_val)
        };

        let string = SharedString::from(string);

        println!("\nSymbols: {}", *sym_val);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(string);
    });

    // Clone references for the number_toggle closure
    let number_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let caps_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);

    window.on_numtoggled(move || {
        let app = number_toggle.upgrade().unwrap();

        // Mutate the nums flag through the RefCell
        let mut nums_value = num_clone.borrow_mut();
        *nums_value = !*nums_value; // Toggle the value

        // Use the updated values
        let string = if *word_clone.borrow() {
            warray(
                *len_clone.borrow(),
                *caps_clone.borrow(),
                *nums_value,
                *sym_clone.borrow(),
                '-',
            )
        } else {
            charray(*len_clone.borrow(), *caps_clone.borrow(), *nums_value, *sym_clone.borrow())
        };
        let string = SharedString::from(string);

        println!("\nNumbers: {}", *nums_value);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(string);
    });

    let capital_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let cap_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);

    window.on_captoggled(move || {
        let app = capital_toggle.upgrade().unwrap();

        let mut caps_val = cap_clone.borrow_mut();
        *caps_val = !*caps_val;

        let string = if *word_clone.borrow() {
            warray(*len_clone.borrow(), *caps_val, *num_clone.borrow(), *sym_clone.borrow(), '-')
        } else {
            charray(*len_clone.borrow(), *caps_val, *num_clone.borrow(), *sym_clone.borrow())
        };
        let string = SharedString::from(string);

        println!("\nCapitls : {}", *caps_val);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(string);
    });

    let word_toggle = window.as_weak();
    let len_clone = Rc::clone(&length);
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let cap_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);

    window.on_wortoggled(move || {
        let app = word_toggle.upgrade().unwrap();
        let mut wor_val = word_clone.borrow_mut();
        *wor_val = !*wor_val;

        let string = if *wor_val {
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
        let string = SharedString::from(string);

        println!("\nWord : {}", *wor_val);
        app.set_copy_state(SharedString::from("Copy Password"));
        app.set_result(string);
    });

    let copy_btn = window.as_weak();
    window.on_copy_clicked(move || {
        let app = copy_btn.upgrade().unwrap();
        app.set_copy_state(SharedString::from("Copied!"));

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let string = app.get_result().to_string();

        ctx.set_contents(string.to_owned()).unwrap();
    });


    window.run().unwrap();
}
