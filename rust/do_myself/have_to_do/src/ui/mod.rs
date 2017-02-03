use super::manager::Manager;
use gtk::prelude::*;
use gtk::{self, Builder, Window};

macro_rules! clone {
    (@param _) => (_);
    (@param $x:ident)=>($x);
    ($($n:ident),+ => move || $body:expr)=>(
        {
            $( let $n =$n.clone();)+
            move || $body
        }
    ($($n:ident),+ => move |$($p::tt),+| &body:expr) => (
        {
            $(let $n=$n.clone(); )+
            move || $body
        }
    );
);
}
pub fn user_interface(file_name: &'static str) {
    match gtk::init() {
        Ok(_) => {
            println!("gtk initialize complete");
        }
        Err(_) => {
            panic!("fial to initialize");
        }
    }
    let show_file = Manager::open(file_name);
    let glade_src = include_str!("have_to_do.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: Window = builder.get_object("window1").unwrap();
    let manage_name = builder.get_object("manage_name").unwrap();

}
