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

fn try_enable_kb_enhancement() -> (impl Drop, Option<std::io::Error>) {
    let res = execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
        )
    );
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            if self.0 {
                execute!(stdout(), PopKeyboardEnhancementFlags).unwrap();
            }
        }
    }
    (Guard(res.is_ok()), res.err())
}

fn play_loop() {
    const DBG: bool = false;

    let (_guard, e) = try_enable_kb_enhancement();

    if DBG && let Some(e) = e {
        dbg!(e);
    }

    let mut play_machine = play_machine::PlayMachine::new();
    loop {
        let event = read().unwrap();
        if let Event::Key(k) = event {
            if DBG {
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

fn main() {
    terminal::enable_raw_mode().unwrap();
    play_loop();
    terminal::disable_raw_mode().unwrap();
}
