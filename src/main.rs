#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::KeyEventHandler, config::Config, helpers::index_selectors, manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More,
};

use simplelog::{LevelFilter, SimpleLogger};

const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "rofi -show run";
const BROWSER: &str = "firefox";
const EDITOR: &str = "emacsclient -c";

fn main() -> penrose::Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    }

    let config = Config::default()
        .builder()
        .workspaces(vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"])
        .gap_px(5)
        .bar_height(35)
        .build()
        .unwrap();
    let key_bindings = gen_keybindings! {
        "M-d" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);
        "M-b" => run_external!(BROWSER);
        "M-q" => run_external!(EDITOR);

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

        "M-1" => run_internal!(focus_workspace, &penrose::Selector::Index(0));
        "M-2" => run_internal!(focus_workspace, &penrose::Selector::Index(1));
        "M-3" => run_internal!(focus_workspace, &penrose::Selector::Index(2));
        "M-4" => run_internal!(focus_workspace, &penrose::Selector::Index(3));
        "M-5" => run_internal!(focus_workspace, &penrose::Selector::Index(4));
        "M-6" => run_internal!(focus_workspace, &penrose::Selector::Index(5));
        "M-7" => run_internal!(focus_workspace, &penrose::Selector::Index(6));
        "M-8" => run_internal!(focus_workspace, &penrose::Selector::Index(7));
        "M-9" => run_internal!(focus_workspace, &penrose::Selector::Index(8));

        "M-S-1" => run_internal!(client_to_workspace, &penrose::Selector::Index(0));
        "M-S-2" => run_internal!(client_to_workspace, &penrose::Selector::Index(1));
        "M-S-3" => run_internal!(client_to_workspace, &penrose::Selector::Index(2));
        "M-S-4" => run_internal!(client_to_workspace, &penrose::Selector::Index(3));
        "M-S-5" => run_internal!(client_to_workspace, &penrose::Selector::Index(4));
        "M-S-6" => run_internal!(client_to_workspace, &penrose::Selector::Index(5));
        "M-S-7" => run_internal!(client_to_workspace, &penrose::Selector::Index(6));
        "M-S-8" => run_internal!(client_to_workspace, &penrose::Selector::Index(7));
        "M-S-9" => run_internal!(client_to_workspace, &penrose::Selector::Index(8));

        // TODO: Make this work, instead of the abomination above!!
        // map: { "1", "2", "3", "4", "5", "6", "7", "8", "9"} to index_selectors(9) => {
        //     "M-{}" => focus_workspace(REF);
        //     "M-S-{}" => client_to_workspace(REF);
        // };
    };

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map! {})
}
