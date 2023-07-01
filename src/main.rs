use mki::{bind_key, Action, Keyboard };
use std::sync::RwLock;
use std::thread;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ACTIVE: RwLock<bool> = RwLock::new(false);
}


fn toggle () {
    let act: bool = ACTIVE.read().unwrap().to_owned();
    let mut new_act = ACTIVE.write().unwrap();
    *new_act = !act;
}

fn main () {

    bind_key(Keyboard::G, Action::handle_kb(|_| toggle() ));
    bind_key(Keyboard::Q, Action::handle_kb(|_| std::process::exit(0) ));

    loop {

        if ACTIVE.read().unwrap().to_owned() { mki::Mouse::release(&mki::Mouse::Left); }

        thread::sleep(Duration::from_millis(1));
    }
}