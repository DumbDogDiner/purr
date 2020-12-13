use cursive::views::{Dialog, TextView};

pub fn run_gui() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("../../assets/style.toml"))
        .unwrap();

    siv.add_layer(
        Dialog::around(TextView::new(
            "This TUI is currently WIP and cannot be used at this time.",
        ))
        .title("Purr TUI")
        .button("Quit", |s| s.quit()),
    );

    siv.run();
}
