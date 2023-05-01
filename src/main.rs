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

fn bot_loop(app: &mut App, actions: Vec<BotAction>) {
    let mut i = 0;
    loop {
        let current_action = &actions[i];
        match current_action {
            BotAction::Sleep(duration) => std::thread::sleep((*duration).into()),
            BotAction::MouseTo(_, _) => {}
            BotAction::KeyStroke(key) => {
                app.send_key_up(WPARAM(*key as usize));
                app.send_key_down(WPARAM(*key as usize));
            }
        }
        i = (i + 1) % actions.len();
    }
}

fn mimic_loop(app: &mut App, config: Config) {
    let Config { mode: Mode::Mimic(delay), layout, ..  } = config else {
        panic!("Expected Mimic mode");
    };

    let delay: Duration = delay.into();
    loop {
        if matches!(layout, Always) {
            if app.swap_windows() {
                app.foreground();
            }
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

    let mut app = App::new(config.clone());

    if matches!(&config.layout, Init | Always) {
        app.layout_windows();
        app.foreground();
    }
    if let Bot(actions) = config.mode {
        bot_loop(&mut app, actions);
    } else if let Mimic(_) = config.mode {
        mimic_loop(&mut app, config);
    }

    Ok(())
}
