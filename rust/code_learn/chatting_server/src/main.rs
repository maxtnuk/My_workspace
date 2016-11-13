extern crate chatting_server;
use chatting_server as chat_p;
use std::thread;
fn main() {
    thread::spawn(||{
        chat_p::chat_client();
    });
    chat_p::chat_server();
}
