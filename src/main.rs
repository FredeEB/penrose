#[macro_use]
extern crate penrose;

use penrose::{
    core::config::Config, logging_error_handler, xcb::new_xcb_backed_window_manager, Backward,
    core::helpers::index_selectors,
    Forward, Less, More,
};

use simplelog::{LevelFilter, SimpleLogger};

const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "rofi -show run";
const BROWSER: &str = "firefox";

fn main() -> penrose::Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    }

    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .gap_px(5)
        .bar_height(35)
        .focused_border("#5c5856")
        .unwrap()
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

        refmap [ 1..10 ] in {
            "M-{}"  => focus_workspace [ index_selectors(9) ];
            "M-S-{}"  => client_to_workspace [ index_selectors(9) ];
        };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})
}
