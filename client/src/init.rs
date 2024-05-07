use crate::{chatbox::menjubar, func::invt, move_top, Ip};
use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{EditView, TextView},
    Cursive, CursiveRunnable,
};
use futures::executor::block_on;
use reqwest::Client;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};


pub fn theme_init(obj: &mut CursiveRunnable, rd: impl AsRef<Path>){
    if let Err(e)= obj.load_theme_file(rd){
        obj.add_layer(cursive::views::Dialog::new().content(TextView::new("Theme Not Loaded")))
    }
    
}
pub fn internet_init(obj: &mut CursiveRunnable) {
    obj.set_user_data(Ip("".to_string()));
    obj.add_layer(
        cursive::views::Dialog::new()
            .title("Please enter the IP address of the host server")
            .content(
                cursive::views::EditView::new()
                    .on_submit(show_popup_init)
                    .with_name("name")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| view.get_content())
                    .unwrap();
                show_popup_init(s, &name);
            }),
    );
}
pub fn internett_init(obj: &mut Cursive) {
    obj.set_user_data(Ip("".to_string()));
    obj.add_layer(
        cursive::views::Dialog::new()
            .title("Please enter the IP address of the host server")
            .content(
                cursive::views::EditView::new()
                    .on_submit(show_popup_init)
                    .with_name("name")
                    .fixed_width(20),
            )
            .button("Ok", |s| {
                let name = s
                    .call_on_name("name", |view: &mut EditView| view.get_content())
                    .unwrap();
                show_popup_init(s, &name);
            }),
    );
}
fn show_popup_init(s: &mut Cursive, name: &str) {
    s.add_layer(cursive::views::Dialog::new().title("Connecting..."));
    if !conect(s, name) {
        s.pop_layer();
        s.pop_layer();
        s.add_layer(
            cursive::views::Dialog::new()
                .title("Connection failed, Enter again.")
                .content(
                    cursive::views::EditView::new()
                        .on_submit(show_popup_init)
                        .with_name("name")
                        .fixed_width(20),
                )
                .button("Ok", |s| {
                    let name = s
                        .call_on_name("name", |view: &mut EditView| view.get_content())
                        .unwrap();
                    show_popup_init(s, &name)
                }),
        );
    } else {
        s.pop_layer();
        s.pop_layer();
        s.add_layer(
            cursive::views::Dialog::new()
                .title("Connection Succeeded")
                .button("Ok", |s| {
                    s.pop_layer();
                }),
        );
        s.with_user_data(|data: &mut Ip| *data = Ip(format!("http://{}", name)));
    }
}
fn conect(s: &mut Cursive, site: &str) -> bool {
    if true {
        return true;
    }
    s.add_layer(cursive::views::Dialog::new().content(cursive::views::TextView::new("Connecting")));
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("http://{}", site))
        .body("Hello World")
        .send();
    s.pop_layer();
    res.is_ok()
}
#[derive(Deserialize)]
pub struct KeySet {
    quit: char,
    inventory: char,
    console: char,
    close: char,
}
pub fn initctrl(obj: &mut CursiveRunnable) {
    obj.add_global_callback(Key::Up, |s| move_top(s, 0, -1));
    obj.add_global_callback(Key::Left, |s| move_top(s, -1, 0));
    obj.add_global_callback(Key::Down, |s| move_top(s, 0, 1));
    obj.add_global_callback(Key::Right, |s| move_top(s, 1, 0));
    obj.add_global_callback(Key::Esc, |s| menjubar(s));
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
    obj.add_global_callback('`', |s| s.toggle_debug_console());
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
