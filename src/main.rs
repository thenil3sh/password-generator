use std::rc::Rc;
use std::cell::RefCell;
use slint::SharedString;
mod func;
use func::{charray, warray};

slint::slint!{
    import { Button, VerticalBox, Switch, GridBox } from "std-widgets.slint";
    export component Window inherits Window {
        in property <string> result : " ";
        callback symtoggled <=> symbols.toggled;
        callback numtoggled <=> numbers.toggled;
        callback captoggled <=> capitals.toggled;
        callback wortoggled <=> words.toggled;


        preferred-height: 300px;
        preferred-width : 700px;
        min-width: 300px;
        VerticalLayout {
        Rectangle {
            preferred-height: 200px;
            preferred-width: 400px;
            Text { font-size : 50px; text: result;}
        }      
        GridLayout {
            Row {
                symbols := Switch {
                    width : 200px;
                    height : 50px;
                    text : "Symbols";
                    }
            numbers := Switch {
                    width : 200px;
                    height : 50px;
                    text : "Numbers";
                        }
                }
            Row {
                capitals := Switch {
                    width : 200px;
                    height : 50px;
                    text : "Capital";
                    }

                words := Switch {
                    width : 200px;
                    height : 50px;
                    text : "Words";
                }

            }
            }
        }  
    }
}



fn main() {
    let window = Window::new().unwrap();

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
        let string = charray(15, *caps_clone.borrow(), *num_clone.borrow(), *sym_val);
        let string = SharedString::from(string);

        println!("Symbols: {}", *sym_val);
        println!("Numbers: {}", *num_clone.borrow());
        println!("Capitls: {}", *caps_clone.borrow());

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
        let string = charray(15, *caps_clone.borrow(), *nums_value, *sym_clone.borrow());
        let string = SharedString::from(string);

        println!("Symbols: {}", *sym_clone.borrow());
        println!("Numbers: {}", *nums_value);
        println!("Capitls: {}", *caps_clone.borrow());
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

        let string = charray(15, *caps_val, *num_clone.borrow(), *sym_clone.borrow());
        let string = SharedString::from(string);

        println!("\nSymbols : {}", *sym_clone.borrow());
        println!("Numbers : {}", *num_clone.borrow());
        println!("Capitls : {}", *caps_val);

        app.set_result(string);
    });

    let word_toggle = window.as_weak();
    let sym_clone = Rc::clone(&symbols);
    let num_clone = Rc::clone(&nums);
    let cap_clone = Rc::clone(&caps);
    let word_clone = Rc::clone(&word);

    window.on_captoggled(move || {
        let app =  word_toggle.upgrade().unwrap();
        let wor_val = word_clone.borrow_mut();
        *word_val = !*word_val;

        if word_val {
            warray(15, );
        }
    })

    window.run().unwrap();
}

/*fn main() {
    let window = Window::new().unwrap();
    let mut nums = false;
    let mut symbols = false;
    let mut caps = false;
    let mut symbols = false;



    let symbol_toggle = window.as_weak();
    window.on_symtoggled(move || {
            println!("{symbols}");
            let app = symbol_toggle.upgrade().unwrap();
            println!("{symbols}");
            let string = charray(15, caps, nums, toggle(&mut symbols));
            let string = SharedString::from(string);
            println!("Symbols : {}", symbols);
            println!("Numbers : {}", nums);
            app.set_result(string);



    let number_toggle = window.as_weak();
    window.on_numtoggled(move || {
            let app = number_toggle.upgrade().unwrap();
            let string = SharedString::from(charray(15, caps, toggle(&mut nums), symbols));
            println!("Symbols : {}", symbols);
            println!("Numbers : {}", nums);
            app.set_result(string);}
    );

    window.run().unwrap();
}*/


fn toggle(flag : &mut bool) -> bool {
    *flag = !*flag;
    *flag
}

/*fn bool (flag : AtomicBool) -> bool {
    flag.load(Ordering::SeqCst)
}*/