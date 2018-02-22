#[macro_use]
extern crate stdweb;

use stdweb::web::{Element,document};

use stdweb::web::event::ClickEvent;

use stdweb::web::IParentNode;
use stdweb::web::IEventTarget;


fn main() {
    stdweb::initialize();

    let register_hits_button: Element = document().query_selector( ".press" ).unwrap().unwrap();
    register_hits_button.add_event_listener(move |_: ClickEvent| {
       console!(log, "worked!");
    });

    stdweb::event_loop();
}