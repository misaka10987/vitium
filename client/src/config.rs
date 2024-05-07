use std::{fs::File, io::Write};

use cursive::{
    theme::BorderStyle,
    view::Scrollable,
    views::{Button, LinearLayout},
    Cursive,
};

pub fn theme_adj(obj: &mut Cursive) {
    let thm = obj.current_theme();
    obj.add_layer(cursive::views::Dialog::around(
        LinearLayout::vertical()
            .child(Button::new(format!("shadow = {}", thm.shadow), |s| {
                shadow_adj(s);
            }))
            .child(Button::new(
                format!(
                    "borders = {}",
                    match thm.borders {
                        BorderStyle::Simple => "Simple",
                        BorderStyle::Outset => "Outset",
                        BorderStyle::None => "None",
                    }
                ),
                |s| borders_adj(s),
            ))
            .child(Button::new("Exit", |s| {
                s.pop_layer();
            }))
            .scrollable()
            .scroll_x(true),
    ));
}
fn shadow_adj(obj: &mut Cursive) {
    obj.add_layer(
        cursive::views::Dialog::new()
            .title("Enable shadow or not")
            .button("Enable", |s| {
                s.update_theme(|t| t.shadow = true);
                s.pop_layer();
                refresh(s);
            })
            .button("Disable", |s| {
                s.update_theme(|t| t.shadow = false);
                s.pop_layer();
                refresh(s);
            }),
    );
}
fn borders_adj(obj: &mut Cursive) {
    obj.add_layer(
        cursive::views::Dialog::new()
            .title("Changing the Type of the Borders")
            .button("Simple", |s| {
                s.update_theme(|t| t.borders = BorderStyle::Simple);
                s.pop_layer();
                refresh(s);
            })
            .button("None", |s| {
                s.update_theme(|t| t.borders = BorderStyle::None);
                s.pop_layer();
                refresh(s);
            })
            .button("Outset", |s| {
                s.update_theme(|t| t.borders = BorderStyle::Outset);
                s.pop_layer();
                refresh(s);
            }),
    )
}
fn refresh(obj: &mut Cursive) {
    obj.pop_layer();
    theme_adj(obj);
}
