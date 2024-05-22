use std::{
    fs::{self, File},
    io::Write,
};

use cursive::{
    theme::{
        BaseColor, BorderStyle, Color,
        PaletteColor::{self, *},
    },
    view::Scrollable,
    views::{Button, Dialog, LinearLayout},
    Cursive,
};

use crate::{Pal, Thm, BS};
fn bctobc(frm: BaseColor) -> String {
    match frm {
        BaseColor::Black => "Black".to_string(),
        BaseColor::Red => "Red".to_string(),
        BaseColor::Green => "Green".to_string(),
        BaseColor::Yellow => "Yello".to_string(),
        BaseColor::Blue => "Blue".to_string(),
        BaseColor::Magenta => "Magenta".to_string(),
        BaseColor::Cyan => "Cyan".to_string(),
        BaseColor::White => "White".to_string(),
    }
}
fn ctoc(frm: Color) -> String {
    match frm {
        Color::TerminalDefault => "Default".to_string(),
        Color::Dark(a) => bctobc(a),
        Color::Light(a) => bctobc(a),
        Color::Rgb(a, b, c) => format!("#{:02x?}{:02x?}{:02x?}", a, b, c),
        Color::RgbLowRes(a, b, c) => format!("#{:x?}{:x?}{:x?}", a, b, c),
    }
}
pub fn theme_adj(obj: &mut Cursive) {
    let thm = obj.current_theme();
    let dialog = LinearLayout::vertical().child(Button::new(format!("shadow = {}", thm.shadow), |s| {
        shadow_adj(s);
    }));
    let mut f = |label, color| {
        dialog.add_button(label, move |s| {
            s.update_theme(|t| t.palette[typ] = Color::Dark(color));
            s.pop_layer();
            refresh(s);
        })
    };
    for (label, color) in [
        ("Black", BaseColor::Black),
        ("Red", BaseColor::Red),
        ("Green", BaseColor::Green),
        ("Yellow", BaseColor::Yellow),
        ("Blue", BaseColor::Blue),
        ("Magenta", BaseColor::Magenta),
        ("Cyan", BaseColor::Cyan),
        ("White", BaseColor::White),
    ] {
        f(label, color);
    }
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
            .child(Button::new(
                format!("background = {}", ctoc(thm.palette[Background])),
                |s| colors_adj(s, PaletteColor::Background),
            ))
            .child(Button::new(
                format!("shadow = {}", ctoc(thm.palette[Shadow])),
                |s| colors_adj(s, PaletteColor::Shadow),
            ))
            .child(Button::new(
                format!("background = {}", ctoc(thm.palette[Background])),
                |s| colors_adj(s, PaletteColor::Background),
            ))
            .child(Button::new(
                format!("background = {}", ctoc(thm.palette[Background])),
                |s| colors_adj(s, PaletteColor::Background),
            ))
            .child(Button::new("Save", |s| {
                let thme = s.current_theme();
                let target = Thm {
                    shadow: thme.shadow,
                    borders: match thme.borders {
                        BorderStyle::Simple => BS::Simple,
                        BorderStyle::Outset => BS::Outset,
                        BorderStyle::None => BS::None,
                    },
                    colors: Pal {
                        background: ctoc(thme.palette[Background]),
                        shadow: ctoc(thme.palette[Shadow]),
                        view: ctoc(thme.palette[View]),
                        primary: ctoc(thme.palette[Primary]),
                        secondary: ctoc(thme.palette[Secondary]),
                        tertiary: ctoc(thme.palette[Tertiary]),
                        title_primary: ctoc(thme.palette[TitlePrimary]),
                        title_secondary: ctoc(thme.palette[TitleSecondary]),
                        highlight: ctoc(thme.palette[Highlight]),
                        highlight_inactive: ctoc(thme.palette[Highlight]),
                        highlight_text: ctoc(thme.palette[HighlightText]),
                    },
                };
                match toml::to_string(&target) {
                    Ok(a) => match fs::write("style.toml", a) {
                        Ok(_) => {
                            s.pop_layer();
                            s.add_layer(
                                cursive::views::Dialog::new()
                                    .title("Saved Successfully")
                                    .dismiss_button("OK"),
                            );
                        }
                        Err(_) => s.add_layer(
                            cursive::views::Dialog::new()
                                .title("Failed to Save")
                                .dismiss_button("OK"),
                        ),
                    },
                    Err(_) => s.add_layer(
                        cursive::views::Dialog::new()
                            .title("Serialization Failed")
                            .dismiss_button("OK"),
                    ),
                }
            }))
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
            .title("Change the Type of the Borders")
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
fn colors_adj(obj: &mut Cursive, typ: PaletteColor) {
    let mut dialog = Dialog::new().title(format!(
        "Change the color of {}",
        match typ {
            Background => "background",
            Shadow => "shadow",
            View => "view",
            Primary => "primary",
            Secondary => "secondary",
            Tertiary => "tertiary",
            TitlePrimary => "titleprimary",
            TitleSecondary => "titlesecondary",
            Highlight => "highlight",
            HighlightInactive => "highlight inactive",
            HighlightText => "hightlight text",
        }
    ));
    let mut f = |label, color| {
        dialog.add_button(label, move |s| {
            s.update_theme(|t| t.palette[typ] = Color::Dark(color));
            s.pop_layer();
            refresh(s);
        })
    };
    for (label, color) in [
        ("Black", BaseColor::Black),
        ("Red", BaseColor::Red),
        ("Green", BaseColor::Green),
        ("Yellow", BaseColor::Yellow),
        ("Blue", BaseColor::Blue),
        ("Magenta", BaseColor::Magenta),
        ("Cyan", BaseColor::Cyan),
        ("White", BaseColor::White),
    ] {
        f(label, color);
    }
    obj.add_layer(
        dialog
            .button("Default", move |s| {
                s.update_theme(|t| t.palette[typ] = Color::TerminalDefault);
                s.pop_layer();
                refresh(s);
            })
            .button("Other", move |s| {
                s.update_theme(|t| t.palette[typ] = Color::TerminalDefault);
                s.pop_layer();
                refresh(s);
            }),
    )
}
// fn inputu8(obj: &mut Cursive,mes:String)->u8{
//     obj.add_layer(cursive::views::Dialog::new().title(mes));
// }
fn refresh(obj: &mut Cursive) {
    obj.pop_layer();
    theme_adj(obj);
}
