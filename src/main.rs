use slint::SharedString;
use std::cell::RefCell;
use std::rc::Rc;
mod func;
use func::{charray, warray};
slint::include_modules!();

fn main() {
    let window = AppWindow::new().unwrap();

    // Wrap booleans in Rc<RefCell> to allow shared mutable access
    let symbols = Rc::new(RefCell::new(false));
    let nums = Rc::new(RefCell::new(false));
    let caps = Rc::new(RefCell::new(false));
    let word = Rc::new(RefCell::new(false));

    // Clone references for the symbol_toggle closure
    let sym_toggle = window.as_weak();
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
            warray(15, *caps_clone.borrow(), *num_clone.borrow(), *sym_val, '-')
        } else {
            charray(15, *caps_clone.borrow(), *num_clone.borrow(), *sym_val)
        };

        let string = SharedString::from(string);

        println!("\nSymbols: {}", *sym_val);

        app.set_result(string);
    });

    // Clone references for the number_toggle closure
    let number_toggle = window.as_weak();
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
                15,
                *caps_clone.borrow(),
                *nums_value,
                *sym_clone.borrow(),
                '-',
            )
        } else {
            charray(15, *caps_clone.borrow(), *nums_value, *sym_clone.borrow())
        };
        let string = SharedString::from(string);

        println!("\nNumbers: {}", *nums_value);
        app.set_result(string);
    });

    let capital_toggle = window.as_weak();
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let cap_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);

    window.on_captoggled(move || {
        let app = capital_toggle.upgrade().unwrap();

        let mut caps_val = cap_clone.borrow_mut();
        *caps_val = !*caps_val;

        let string = if *word_clone.borrow() {
            warray(15, *caps_val, *num_clone.borrow(), *sym_clone.borrow(), '-')
        } else {
            charray(15, *caps_val, *num_clone.borrow(), *sym_clone.borrow())
        };
        let string = SharedString::from(string);

        println!("\nCapitls : {}", *caps_val);

        app.set_result(string);
    });

    let word_toggle = window.as_weak();
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
                15,
                *cap_clone.borrow(),
                *num_clone.borrow(),
                *sym_clone.borrow(),
                '-',
            )
        } else {
            charray(
                15,
                *cap_clone.borrow(),
                *num_clone.borrow(),
                *sym_clone.borrow(),
            )
        };
        let string = SharedString::from(string);

        println!("\nWord : {}", *wor_val);
        app.set_result(string);
    });

    window.run().unwrap();
}
