use slint::SharedString;
mod func;
use func::charray;


slint::slint!{
    import { Button, VerticalBox, Switch, GridBox } from "std-widgets.slint";
    export component Window inherits Window {
        in property <string> result : " ";
        callback symtoggled <=> symbols.toggled;
        callback numtoggled <=> numbers.toggled;
        callback captoggled <=> capitals.toggled;



        preferred-height: 300px;
        preferred-width : 600px;
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

            }
            }
        }  
    }
}

fn main() {
    let window = Window::new().unwrap();
    let mut caps = false;
    let mut nums = false;
    let mut symbols = true;



    let symbol_toggle = window.as_weak();
    window.on_symtoggled({
        toggle(&mut symbols);
        
        println!("{symbols}");
        
        move || {
            let app = symbol_toggle.upgrade().unwrap();
            println!("{symbols}");
            let string = charray(15, caps, nums, symbols.clone());
            let string = SharedString::from(string);
            app.set_result(string);
    }
});


    let number_toggle = window.as_weak();
    window.on_numtoggled({
        
        toggle(&mut nums);
        
        move || {
            let app = number_toggle.upgrade().unwrap();
            let string = SharedString::from(charray(15, caps, nums.clone(), symbols.clone()));
            app.set_result(string);}
    });

    window.run().unwrap();
}


fn toggle(switch : &mut bool) {
    *switch = !*switch;
}