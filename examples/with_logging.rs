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
        "This is the default binary (dev/run.rs) that lets you test your Reedline code in the terminal."
    );
    println!("Actions are logged into readline.log file (use dev/logger.rs to change file or logging level).");
    println!("You may check the examples directory for more bins that demonstrate various Reedline features.");

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
