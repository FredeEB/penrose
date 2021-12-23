#[macro_use]
extern crate penrose;

use std::path::PathBuf;
use std::process::Command;
use std::fs::read_dir;

use penrose::{Backward, Forward, Less, More, core::{ 
    hooks::Hook, config::Config, helpers::index_selectors, xconnection::XConn}, 
    logging_error_handler, xcb::{XcbConnection, new_xcb_backed_window_manager}, WindowManager};

use simplelog::{LevelFilter, SimpleLogger};

const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "rofi -show run";
const BROWSER: &str = "firefox";

struct StartupScript {
    dir: PathBuf
}

impl StartupScript {
    fn new(s: impl Into<PathBuf>) -> Self {
        Self { dir: s.into() }
    }
}

impl<X> Hook<X> for StartupScript where X: XConn {
    fn startup(&mut self, _: &mut WindowManager<X>) -> penrose::Result<()> {
        let dir = read_dir(&self.dir)?;
        dir.for_each(|path| {
            if let Ok(path) = path {
                match Command::new(path.path()).status() {
                    Ok(_) => {},
                    Err(e) => println!("Error: {}", e),
                };
            }
        });
        Ok(())
    }
}

fn main() -> penrose::Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    }

    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .bar_height(35)
        .focused_border("#5c5856")
        .unwrap()
        .border_px(1)
        .build()
        .unwrap();

    let key_bindings = gen_keybindings! {
        "M-d" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);
        "M-b" => run_external!(BROWSER);

        "M-S-e" => run_internal!(exit);

        // TODO: Make this work without indices!
        "M-S-h" => run_internal!(client_to_screen, &penrose::Selector::Index(0));
        "M-S-l" => run_internal!(client_to_screen, &penrose::Selector::Index(1));
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-h" => run_internal!(cycle_screen, Backward);
        "M-l" => run_internal!(cycle_screen, Forward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &penrose::Selector::Focused);
        "M-S-q" => run_internal!(kill_client);

        "M-u" => run_internal!(update_max_main, Less);
        "M-i" => run_internal!(update_max_main, More);
        "M-o" => run_internal!(update_main_ratio, More);
        "M-y" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mut user_scripts = PathBuf::from(std::env::var("HOME").expect("HOME Var not set"));
    user_scripts.push(".config/penrose");

    let hooks: Vec<Box<dyn Hook<XcbConnection> + 'static>> = vec![
        Box::new(StartupScript::new("/usr/share/penrose")),
        Box::new(StartupScript::new(user_scripts)),
    ];

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})
}
