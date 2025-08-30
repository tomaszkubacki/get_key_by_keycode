extern crate evdev;
extern crate xkbcommon;

use evdev::EventType;
use std::env;
use xkbcommon::xkb::{self, Keycode};

// evdev constants:
const KEYCODE_OFFSET: u16 = 8;
const KEY_STATE_RELEASE: i32 = 0;
const KEY_STATE_PRESS: i32 = 1;
const KEY_STATE_REPEAT: i32 = 2;

fn main() {
    if env::args().len() < 2 {
        println!("pass input device argument eg. /dev/input/event0");
        return;
    }
    let kbd_path_dev = env::args().nth(1).unwrap();
    let mut device = evdev::Device::open(std::env::args().nth(1).unwrap_or(kbd_path_dev)).unwrap();
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

    let keymap = xkb::Keymap::new_from_names(
        &context,
        "evdev",                                     // rules
        "pc105",                                     // model
        "pl",                                        // layout
        "",                                          // variant
        Some("terminate:ctrl_alt_bksp".to_string()), // options
        xkb::COMPILE_NO_FLAGS,
    )
    .unwrap();

    let mut state = xkb::State::new(&keymap);
    loop {
        for event in device.fetch_events().unwrap() {
            if event.event_type() == EventType::KEY {
                let keycode: Keycode = (event.code() + KEYCODE_OFFSET).into();

                if event.value() == KEY_STATE_REPEAT && !keymap.key_repeats(keycode) {
                    continue;
                }

                if event.value() == KEY_STATE_RELEASE {
                    state.update_key(keycode, xkb::KeyDirection::Up)
                } else {
                    state.update_key(keycode, xkb::KeyDirection::Down)
                };

                // TODO add met key handling
                // Inspect state
                //     if state.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE) {
                //         print!("alt ");
                //     }

                if event.value() == KEY_STATE_PRESS {
                    let keysym = state.key_get_one_sym(keycode);
                    println!("keysym: {} ", xkb::keysym_get_name(keysym));
                }
            }
        }
    }
}
