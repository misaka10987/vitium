use crate::{func::invt, move_top};
use cursive::{event::Key, views::TextView, CursiveRunnable};
use futures::executor::block_on;
use reqwest::Client;
use serde_derive::Deserialize;
use std::{char, fs::File, io::Read, path::Path};
#[derive(Deserialize)]
pub struct KeySet {
    quit: char,
    inventory: char,
    console: char,
    close: char,
}
pub fn config(obj: &mut CursiveRunnable) {
    obj.add_layer(cursive::views::Dialog::new());
}
pub fn initctrl(obj: &mut CursiveRunnable) {
    obj.add_global_callback(Key::Up, |s| move_top(s, 0, -1));
    obj.add_global_callback(Key::Left, |s| move_top(s, -1, 0));
    obj.add_global_callback(Key::Down, |s| move_top(s, 0, 1));
    obj.add_global_callback(Key::Right, |s| move_top(s, 1, 0));
    let path = Path::new("keyboard.toml");
    match File::open(&path) {
        Err(_) => initctrl_init(obj),
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(_) => initctrl_init(obj),
                Ok(_) => {
                    let p: Result<KeySet, toml::de::Error> = toml::from_str(&s);
                    match p {
                        Ok(k) => {
                            obj.add_global_callback(k.quit, |s| s.quit());
                            obj.add_global_callback(k.inventory, |s| {
                                s.add_layer(
                                    cursive::views::Dialog::new()
                                        .content(TextView::new("Opening Inventory...")),
                                );
                                block_on(invt(s, &mut Client::new(), "idk".to_string()));
                            });
                            obj.add_global_callback(k.console, |s| s.toggle_debug_console());
                            obj.add_global_callback(k.close, |s| {
                                s.pop_layer();
                            });
                            obj.add_layer(
                                cursive::views::Dialog::new()
                                    .title("Keyboard Setting Accepted")
                                    .button("Ok", |s| {
                                        s.pop_layer();
                                    }),
                            );
                        }
                        Err(_) => initctrl_init(obj),
                    }
                }
            }
        }
    };
}
pub fn initctrl_init(obj: &mut CursiveRunnable) {
    obj.add_global_callback('q', |s| s.quit());
    obj.add_global_callback('f', |s| {
        block_on(invt(s, &mut Client::new(), "idk".to_string()));
    });
    obj.add_global_callback('~', |s| s.toggle_debug_console());
    obj.add_global_callback('z', |s| {
        s.pop_layer();
    });
    obj.add_layer(
        cursive::views::Dialog::new()
            .title("Keyboard Setting Unaccepted")
            .button("Ok", |s| {
                s.pop_layer();
            }),
    );
}
