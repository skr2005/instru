use crate::MainArgs;
use crate::player::Note;
use crate::player::NotePlayer;
use crossterm::event::KeyEvent;
use crossterm::{
    event::{
        Event, KeyCode, KeyModifiers, KeyboardEnhancementFlags,
        PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags, read,
    },
    execute, terminal,
};
use std::io::stdout;
use web_audio_api::context::AudioContext;

fn ensure_terminal_raw_mode(debug: bool) -> Option<impl Drop> {
    match terminal::is_raw_mode_enabled() {
        Err(e) if debug => {
            dbg!(e);
        }
        Ok(b) if b => return None,
        _ => (),
    }
    terminal::enable_raw_mode()
        .expect("Cannot enable the raw mode of the terminal");
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            terminal::disable_raw_mode().expect(
                "Cannot disable the raw mode of the terminal \
                enabled by this program earlier",
            );
        }
    }
    Some(Guard)
}

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
                execute!(stdout(), PopKeyboardEnhancementFlags).expect(
                    "Cannot pop a keyboard enhancement flag \
                    pushed ealier in the terminal",
                );
            }
        }
    }
    (Guard(err.iter().filter(|e| e.is_none()).count()), err)
}

pub fn play_loop(main_args: &MainArgs) {
    let ctx = main_args.audio_context();

    let _g_raw = ensure_terminal_raw_mode(main_args.debug);
    let (_g_en, e) = try_enable_kb_enhancement();

    if main_args.debug {
        dbg!(e);
    }

    let mut play_machine = PlayMachine::new(ctx, main_args.a_frequency);
    loop {
        let event = read().expect("Cannot read a terminal event");
        let Event::Key(k) = event else {
            continue;
        };
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

struct PlayMachine {
    player: NotePlayer,
    pressing_note_key_state: Option<NoteKeyState>,
    space_on: bool,
    flat: bool,
    sharp: bool,
}

#[derive(Clone, Copy, Debug)]
struct NoteKeyState {
    note: Note,
    ottava: isize,
}

impl PlayMachine {
    pub(crate) fn new(ctx: AudioContext, a_frequency: f32) -> Self {
        Self {
            player: NotePlayer::new(ctx, a_frequency, 0, 0),
            pressing_note_key_state: None,
            space_on: false,
            flat: false,
            sharp: false,
        }
    }

    pub(crate) fn handle_on(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;

        let handle_play = |me: &mut Self, note| {
            let modif_has = |m| k.modifiers.contains(m);
            let ottava = modif_has(KeyModifiers::SHIFT) as isize
                - modif_has(KeyModifiers::ALT) as isize;
            me.player.handle_play(
                note,
                me.sharp as isize - me.flat as isize,
                ottava,
            );
            me.pressing_note_key_state =
                Some(NoteKeyState { note, ottava });
        };

        let handle_non_play = |me: &mut Self| {
            match k.code {
                Char('.') | Char('>') => {
                    me.flat = true;
                }
                Char('/') | Char('?') => {
                    me.sharp = true;
                }
                Char(' ') | Char('s') | Char('S') => {
                    me.space_on = true;
                }
                Up => {
                    me.player.handle_transpose(0, 1);
                }
                Down => {
                    me.player.handle_transpose(0, -1);
                }
                _ => (),
            };
            if let Some(NoteKeyState { note, ottava }) =
                me.pressing_note_key_state
            {
                me.player.handle_play(
                    note,
                    me.sharp as isize - me.flat as isize,
                    ottava,
                );
            }
        };

        if let Char(ch) = k.code
            && let Ok(note) = ch.try_into()
        {
            handle_play(self, note);
            return;
        }
        handle_non_play(self);
    }

    pub(crate) fn handle_off(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;

        let Char(ch) = k.code else {
            return;
        };

        let should_replay = match ch {
            '.' | '>' => {
                self.flat = false;
                true
            }
            '/' | '?' => {
                self.sharp = false;
                true
            }
            ' ' | 's' | 'S' => {
                self.space_on = false;
                if self.pressing_note_key_state.is_none() {
                    self.player.handle_stop();
                }
                false
            }
            _ => {
                if let Ok(ch_note) = Note::try_from(ch)
                    && let Some(NoteKeyState {
                        note: playing_note, ..
                    }) = self.pressing_note_key_state
                    && ch_note == playing_note
                {
                    self.pressing_note_key_state = None;
                    if !self.space_on {
                        self.player.handle_stop();
                    }
                };
                false
            }
        };
        if should_replay
            && let Some(NoteKeyState { note, ottava }) =
                self.pressing_note_key_state
        {
            self.player.handle_play(
                note,
                self.sharp as isize - self.flat as isize,
                ottava,
            );
        }
    }
}
