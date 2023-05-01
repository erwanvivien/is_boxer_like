mod config;
use std::time::Duration;

use is_boxer_like::App;
use is_boxer_like::BotAction;
use is_boxer_like::LayoutOptions::{Always, Init};
use is_boxer_like::Mode::{Bot, Mimic};
use is_boxer_like::WPARAM;
use is_boxer_like::{Config, Mode};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    config: Option<String>,
}

fn bot_loop(app: &mut App) {
    let mut last_sleep = None;

    let mut i = 0;
    loop {
        if app.global_shortcuts() {
            return;
        }

        let actions = &app.config.bot_action;
        let current_action = &actions[i];

        match current_action {
            BotAction::Sleep(duration) => {
                if last_sleep.is_none() {
                    last_sleep = Some(std::time::Instant::now());
                } else if last_sleep.unwrap().elapsed() >= (*duration).into() {
                    last_sleep = None;
                }

                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            BotAction::MouseTo(_, _) => {}
            BotAction::KeyStroke(key) => {
                app.send_key_up(WPARAM(*key as usize));
                app.send_key_down(WPARAM(*key as usize));
            }
        }

        if last_sleep.is_none() {
            i = (i + 1) % actions.len();
        }
    }
}

fn mimic_loop(app: &mut App) {
    let Config {
        mimic_timer: delay,
        layout,
        ..
    } = app.config.clone();

    let delay: Duration = delay.into();
    loop {
        if matches!(layout, Always) {
            if app.swap_windows() {
                app.foreground();
            }
        }

        if app.global_shortcuts() {
            return;
        }

        app.mimic();
        std::thread::sleep(delay);
    }
}

fn main() -> Result<(), eframe::Error> {
    // let mut app = App::new();
    // app.update_windows("warcraft");

    // app.send_mouse(WPARAM(0), 2520, 1340);

    // return Ok(());

    let Args { config } = Args::parse();
    if let Some(config_path) = &config {
        println!("Using config file: {}", config_path);
    } else {
        println!("No config file, using mimic with 10ms delay");
    };

    let config = if let Some(config) = config {
        Config::load(config)
    } else {
        Config::default()
    };

    if config.window_name.is_empty() {
        panic!("No window name specified");
    }

    let mut app = App::new(config);

    if matches!(&app.config.layout, Init | Always) {
        app.layout_windows();
        app.foreground();
    }

    loop {
        if let Bot = app.config.mode {
            bot_loop(&mut app);
        } else if let Mimic = app.config.mode {
            mimic_loop(&mut app);
        } else {
            app.global_shortcuts();
        }
    }
}
