use cursive::CursiveRunnable;
use futures::executor::block_on;
use reqwest::Client;
use serde::Deserialize;
use std::{char, fs::File, io::Read, path::Path};

use crate::{func::invt, move_top};
#[derive(Deserialize)]
pub struct KeySet {
    quit: u32,
    up: u32,
    down: u32,
    left: u32,
    right: u32,
    inventory: u32,
    console: u32,
    close: u32,
}
pub fn config(obj: &mut CursiveRunnable) {
    obj.add_layer(cursive::views::Dialog::new());
}
pub fn initctrl(obj: &mut CursiveRunnable) {
    let path = Path::new("keyboard.json");
    match File::open(&path) {
        Err(_) => initctrl_init(obj),
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(_) => initctrl_init(obj),
                Ok(_) => {
                    let p: Result<KeySet, serde_json::Error> = serde_json::from_str(&s);
                    match p {
                        Ok(k) => unsafe {
                            obj.add_global_callback(char::from_u32_unchecked(k.quit), |s| s.quit());
                            obj.add_global_callback(char::from_u32_unchecked(k.up), |s| {
                                move_top(s, 0, -1)
                            });
                            obj.add_global_callback(char::from_u32_unchecked(k.left), |s| {
                                move_top(s, -1, 0)
                            });
                            obj.add_global_callback(char::from_u32_unchecked(k.down), |s| {
                                move_top(s, 0, 1)
                            });
                            obj.add_global_callback(char::from_u32_unchecked(k.right), |s| {
                                move_top(s, 1, 0)
                            });
                            obj.add_global_callback(char::from_u32_unchecked(k.inventory), |s| {
                                block_on(invt(s, &mut Client::new(), "idk".to_string()));
                            });
                            obj.add_global_callback(char::from_u32_unchecked(k.console), |s| {
                                s.toggle_debug_console()
                            });
                            obj.add_global_callback(char::from_u32_unchecked(k.close), |s| {
                                s.pop_layer();
                            });
                            obj.add_layer(
                                cursive::views::Dialog::new()
                                    .title("Keyboard Setting Accepted")
                                    .button("Ok", |s| {
                                        s.pop_layer();
                                    }),
                            );
                        },
                        Err(_) => initctrl_init(obj),
                    }
                }
            }
        }
    };
}
pub fn initctrl_init(obj: &mut CursiveRunnable) {
    obj.add_global_callback('q', |s| s.quit());
    obj.add_global_callback('w', |s| move_top(s, 0, -1));
    obj.add_global_callback('a', |s| move_top(s, -1, 0));
    obj.add_global_callback('s', |s| move_top(s, 0, 1));
    obj.add_global_callback('d', |s| move_top(s, 1, 0));
    obj.add_global_callback('f', |s| {
        block_on(invt(s, &mut Client::new(), "idk".to_string()));
    });
    obj.add_global_callback('~', |s| s.toggle_debug_console());
    obj.add_global_callback('z', |s| {
        s.pop_layer();
    });
}
