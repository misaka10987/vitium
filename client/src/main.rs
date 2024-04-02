use cursive::{
    event::{Event, Key},
    views::{Dialog, TextView},
    CursiveRunnable,
};

fn map_hjkl(r: &mut CursiveRunnable) {
    r.set_on_pre_event(Event::Char('h'), |c| c.on_event(Event::Key(Key::Left)));
    r.set_on_pre_event(Event::Char('j'), |c| c.on_event(Event::Key(Key::Down)));
    r.set_on_pre_event(Event::Char('k'), |c| c.on_event(Event::Key(Key::Up)));
    r.set_on_pre_event(Event::Char('l'), |c| c.on_event(Event::Key(Key::Right)));
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::around(TextView::new("Hello Dialog!"))
            .title("Cursive")
            .button("Quit", |s| s.quit()),
    );

    siv.clear_global_callbacks(Event::CtrlChar('c'));

    siv.set_on_pre_event(Event::CtrlChar('c'), |s| {
        s.add_layer(
            Dialog::text("Do you want to quit?")
                .button("Yes", |s| s.quit())
                .button("No", |s| {
                    s.pop_layer();
                }),
        );
    });

    map_hjkl(&mut siv);

    // Starts the event loop.
    siv.run();
}
