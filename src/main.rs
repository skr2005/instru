use clap::Parser;
use std::io::stdout;

use crossterm::{
    event::{
        Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags, read,
    },
    execute, terminal,
};

mod play_machine;
mod tune_player;

fn try_enable_kb_enhancement() -> (impl Drop, [Option<std::io::Error>; 3])
{
    let try_flags = [
        KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
        KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES,
    ];

    let err = try_flags.map(|f| {
        execute!(stdout(), PushKeyboardEnhancementFlags(f)).err()
    });

    struct Guard(usize);
    impl Drop for Guard {
        fn drop(&mut self) {
            for _ in 0..self.0 {
                execute!(stdout(), PopKeyboardEnhancementFlags).unwrap();
            }
        }
    }
    (Guard(err.iter().filter(|e| e.is_none()).count()), err)
}

fn play_loop(main_args: &MainArgs) {
    let (_guard, e) = try_enable_kb_enhancement();

    if main_args.debug {
        dbg!(e);
    }

    let mut play_machine =
        play_machine::PlayMachine::new(main_args.a_frequency);
    loop {
        let event = read().unwrap();
        if let Event::Key(k) = event {
            if main_args.debug {
                dbg!(k);
            }
            if k.is_press() || k.is_repeat() {
                if k.code.is_esc()
                    || (k.modifiers.contains(KeyModifiers::CONTROL)
                        && (k.code == KeyCode::Char('c')
                            || k.code == KeyCode::Char('C')
                            || k.code == KeyCode::Char('z')
                            || k.code == KeyCode::Char('Z')
                            || k.code == KeyCode::Char('d')
                            || k.code == KeyCode::Char('D')))
                {
                    return;
                }
                play_machine.handle_on(k);
            }
            if k.is_release() {
                play_machine.handle_off(k);
            }
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct MainArgs {
    /// Enable logging and other things for debug use
    #[arg(long)]
    debug: bool,

    /// The frequency initially matching the key A
    ///
    /// Set it to 440 if you want the key C to initially match middle C.
    #[arg(long, short, default_value_t = 880.)]
    a_frequency: f32,
}

fn main() {
    let args = MainArgs::parse();

    terminal::enable_raw_mode().unwrap();
    play_loop(&args);
    terminal::disable_raw_mode().unwrap();
}
