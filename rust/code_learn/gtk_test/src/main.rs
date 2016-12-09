extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};
fn main() {
    match gtk::init() {
        Ok(_) => {
            println!("gtk init complete");
        }
        Err(_) => {
            panic!("fail to init");
        }
    }
    let window = Window::new(WindowType::Toplevel);
    window.set_title("first gtk program");
    window.set_default_size(350, 70);
    let button = Button::new_with_label("click me");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|_| {
        println!("cliked");
    });

    gtk::main();
}
