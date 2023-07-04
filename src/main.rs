use mki::{bind_key, Action, Keyboard };
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use imgui::*;

mod support;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ACTIVE: RwLock<bool> = RwLock::new(false);
    static ref BUTTON: RwLock<bool> = RwLock::new(true);
    static ref DELAY: RwLock<u64> = RwLock::new(1);
}


fn click () {

    toggle();

    let side: bool = BUTTON.read().unwrap().to_owned();
    let delay: u64 = DELAY.read().unwrap().to_owned();

    while ACTIVE.read().unwrap().to_owned() {

        if side {
            mki::Mouse::release(&mki::Mouse::Left);
            mki::Mouse::release(&mki::Mouse::Left); 
        }
        else { mki::Mouse::release(&mki::Mouse::Right); }

        thread::sleep(Duration::from_millis(delay));
    }
}

fn toggle () {
    let act: bool = ACTIVE.read().unwrap().to_owned();
    let mut new_act = ACTIVE.write().unwrap();
    *new_act = !act;
}

fn toggle_button () {
    let but: bool = BUTTON.read().unwrap().to_owned();
    let mut new_but = BUTTON.write().unwrap();
    *new_but = !but;
}

fn main () {

    let system = support::init(file!());

    let mut value = 0;
    let start = ["Start(G)", "Stop(G)"];
    let mut value2 = 0;
    let button = ["Left Click", "Right Click"];

    let mut delay_ms: i32 = 1;

    bind_key(Keyboard::G, Action::handle_kb(|_| { click() } ));
    bind_key(Keyboard::Q, Action::handle_kb(|_| std::process::exit(0) ));
    println!("\nUse the <G> key to start/stop clicker. \nUse the <Q> key to exit to terminal.\n");


    system.main_loop(move |_, ui| {
        ui.window("Auto-Clicker")
            .size([500.0, 360.0], Condition::FirstUseEver)
            .build(|| {
                ui.text_wrapped("Use your keyboard to start/stop clicking when this window is not in focus");
                if ui.button(start[value]) || ui.is_key_pressed(Key::G) {
                    value += 1;
                    value %= 2;
                    click();
                }
                ui.separator();

                if ui.button(button[value2]) {
                    value2 += 1;
                    value2 %= 2;
                    toggle_button();
                }

                ui.separator();

                if ui.input_int("Delay in ms: ", &mut delay_ms).build() {
                    if delay_ms < 1 { delay_ms = 1 }
                    let mut new_del = DELAY.write().unwrap();
                    *new_del = delay_ms as u64;
                }

                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));

                if ui.button("Quit") || ui.is_key_pressed(Key::Q) {
                    std::process::exit(0);
                }
            });
    });
    
}