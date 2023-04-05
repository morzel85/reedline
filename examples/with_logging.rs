mod utils;

use crossterm::cursor::CursorShape;
use reedline::CursorConfig;
use reedline::{
    default_vi_insert_keybindings, default_vi_normal_keybindings, DefaultPrompt, Reedline, Signal,
    Vi,
};
use std::io;

fn main() -> io::Result<()> {
    println!("\nHello, Reedline Developer!");
    println!(
        "This binary lets you test Reedline in the terminal while actions are logged into reedline.log file."
    );
    println!("You can change the file and log level settings in the logger_init function (examples/utils/mod.rs).");

    utils::logger_init();

    let cursor_config = CursorConfig {
        vi_insert: Some(CursorShape::Line),
        vi_normal: Some(CursorShape::Block),
        emacs: None,
    };

    let mut line_editor = Reedline::create().with_cursor_config(cursor_config);

    let vi_mode = matches!(std::env::args().nth(1), Some(x) if x == "--vi");

    if vi_mode {
        println!("--vi parameter set, using Vi mode.");
        line_editor = line_editor
            .with_edit_mode(Box::new(Vi::new(
                default_vi_normal_keybindings(),
                default_vi_insert_keybindings(),
            )))
            .with_buffer_editor("vi".into(), "nu".into());
    } else {
        println!("--vi parameter not set, using default (Emacs) mode.");
    }

    println!("Press Ctrl-C to quit.\n");

    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt)?;

        if let Signal::CtrlC = sig {
            println!("\nCtrl-C received, bye!");
            break Ok(());
        }
    }
}
