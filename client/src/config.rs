use std::fs;

use cursive::{
    theme::{
        BaseColor, BorderStyle, Color,
        PaletteColor::{self, *},
    },
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
fn pctopc(frm: PaletteColor) -> String {
    match frm {
        Background => "background".to_string(),
        Shadow => "shadow".to_string(),
        View => "view".to_string(),
        Primary => "primary".to_string(),
        Secondary => "secondary".to_string(),
        Tertiary => "tertiary".to_string(),
        TitlePrimary => "titleprimary".to_string(),
        TitleSecondary => "titlesecondary".to_string(),
        Highlight => "highlight".to_string(),
        HighlightInactive => "highlight inactive".to_string(),
        HighlightText => "hightlight text".to_string(),
    }
}
pub fn theme_adj(obj: &mut Cursive) {
    let thm = obj.current_theme();
    obj.add_layer(
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
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::Background),
                    ctoc(thm.palette[PaletteColor::Background])
                ),
                move |s| colors_adj(s, PaletteColor::Background),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::Shadow),
                    ctoc(thm.palette[PaletteColor::Shadow])
                ),
                move |s| colors_adj(s, PaletteColor::Shadow),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::View),
                    ctoc(thm.palette[PaletteColor::View])
                ),
                move |s| colors_adj(s, PaletteColor::View),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::Primary),
                    ctoc(thm.palette[PaletteColor::Primary])
                ),
                move |s| colors_adj(s, PaletteColor::Primary),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::Secondary),
                    ctoc(thm.palette[PaletteColor::Secondary])
                ),
                move |s| colors_adj(s, PaletteColor::Secondary),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::Tertiary),
                    ctoc(thm.palette[PaletteColor::Tertiary])
                ),
                move |s| colors_adj(s, PaletteColor::Tertiary),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::TitlePrimary),
                    ctoc(thm.palette[PaletteColor::TitlePrimary])
                ),
                move |s| colors_adj(s, PaletteColor::TitlePrimary),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::TitleSecondary),
                    ctoc(thm.palette[PaletteColor::TitleSecondary])
                ),
                move |s| colors_adj(s, PaletteColor::TitleSecondary),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::Highlight),
                    ctoc(thm.palette[PaletteColor::Highlight])
                ),
                move |s| colors_adj(s, PaletteColor::Highlight),
            ))
            .child(Button::new(
                format!(
                    "{} = {}",
                    pctopc(PaletteColor::HighlightInactive),
                    ctoc(thm.palette[PaletteColor::HighlightInactive])
                ),
                move |s| colors_adj(s, PaletteColor::HighlightInactive),
            ))
            .child(Button::new("Return to Default",|s| {s.set_theme(cursive::theme::load_default())}))
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
            })),
    );
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
    let mut dialog = Dialog::new().title(format!("Change the color of {}", pctopc(typ)));
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
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    )
}
fn refresh(obj: &mut Cursive) {
    obj.pop_layer();
    theme_adj(obj);
}
