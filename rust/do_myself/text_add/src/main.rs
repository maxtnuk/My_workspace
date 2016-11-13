extern crate native_windows_gui as nwg;

use nwg::Ui;
use nwg::events::EventCallback;
use nwg::controls::{Button, Window, TextInput};
use nwg::actions::{Action, ActionReturn};
use nwg::actions::helper as actions_help;
use nwg::constants::{HTextAlign, VTextAlign, MessageButtons, MessageIcons};

fn setup_controls(ui: &mut Ui<&'static str>) {
    let main_window = Window {
        caption: "Hello".to_string(),
        size: (200, 200),
        position: (100, 100),
        visible: true,
        resizable: false,
        exit_on_close: true
    };

    let name_input = TextInput {
        text: "".to_string(),
        size: (180, 20),
        position: (10, 10),
        parent: "MainWindow",
        placeholder: Some("Your name Here".to_string()),
        text_align: HTextAlign::Left,
        password: false,
        readonly: false
    };

    let hello_button = Button {
        text: "Say Hello!".to_string(),
        size: (180, 130),
        position: (10, 50),
        parent: "MainWindow",
        text_align: (HTextAlign::Center, VTextAlign::Center),
    };

    ui.new_control("MainWindow", main_window).unwrap();
    ui.new_control("Name", name_input).unwrap();
    ui.new_control("Hello", hello_button).unwrap();
}

fn main() {
    let mut ui: Ui<&'static str> = Ui::new();

    setup_controls(&mut ui);

    ui.bind("Hello", "SayHello", EventCallback::Click(Box::new(|ui, caller|{
        println!("Caller is: {:?}", caller);
        if let Ok(ActionReturn::Text(name)) = ui.exec("Name", Action::GetText) {
            let msg = actions_help::message(
                "Hello!", format!("Hello {}!", name),
                MessageButtons::Ok, MessageIcons::None
            );
            ui.exec("MainWindow", msg).unwrap();
        }

    }))).unwrap();

    nwg::dispatch_events();
}
