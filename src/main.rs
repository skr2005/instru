use std::io::stdout;

use crossterm::{
    event::{
        Event, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags, read,
    },
    execute, terminal,
};

mod play_machine;
mod tune_player;

fn try_enable_kb_enhancement() -> (impl Drop, bool) {
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
    (Guard(res.is_ok()), res.is_ok())
}

fn play_loop() {
    const DBG: bool = false;
    let _guard = try_enable_kb_enhancement();
    let mut play_machine = play_machine::PlayMachine::new();
    loop {
        let event = read().unwrap();
        if let Event::Key(k) = event {
            if DBG {
                dbg!(k);
            }
            if k.is_press() || k.is_repeat() {
                if k.code.is_esc() {
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
