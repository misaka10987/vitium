use cursive::view::{Nameable, Resizable};
use cursive::views::EditView;
use cursive::Cursive;
use cursive::CursiveRunnable;

pub struct Data {
    pub server_ip: String,
}

pub fn scr_init(obj: &mut CursiveRunnable, rd: String) {
    let a = obj.load_theme_file(rd);
    if !a.is_err() {
        a.unwrap();
    }
    obj.set_user_data(Data {
        server_ip: "".to_string(),
    });
    obj.add_layer(
        cursive::views::Dialog::new()
            .title("Please enter the IP address of the host server".to_string())
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
    if !conect(name) {
        s.pop_layer();
        s.pop_layer();
        s.add_layer(
            cursive::views::Dialog::new()
                .title("Connection failed, Enter again.".to_string())
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
        s.with_user_data(|data: &mut Data| data.server_ip = name.to_string());
    }
}
fn conect(site: &str) -> bool {
    let client = reqwest::blocking::Client::new();
    let res = client.post(site.to_string()).body("Hello World").send();
    res.is_ok()
}
