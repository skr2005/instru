use crate::{
    MainArgs,
    player::{Note, NotePlayer},
};
use evdev::{Device, KeyCode};
use web_audio_api::context::AudioContext;

pub fn play_loop(main_args: &MainArgs) {
    let ctx = main_args.audio_context();

    let Some(keyboard) = main_args.keyboard.as_ref() else {
        panic!(
            "The path of keyboard device must be specified in this mode"
        )
    };
    let mut keyboard = Device::open(keyboard)
        .expect("Cannot access the specified device");

    if main_args.debug {
        dbg!(keyboard.name());
        dbg!(keyboard.physical_path());
        dbg!(keyboard.unique_name());
        dbg!(keyboard.input_id().bus_type());
        dbg!(keyboard.get_auto_repeat());

        let s = keyboard
            .supported_keys()
            .expect("The specified device should support some keys");

        let supported_keys = [
            KeyCode::KEY_A,
            KeyCode::KEY_B,
            KeyCode::KEY_C,
            KeyCode::KEY_D,
            KeyCode::KEY_E,
            KeyCode::KEY_F,
            KeyCode::KEY_G,
            KeyCode::KEY_S,
            KeyCode::KEY_SPACE,
            KeyCode::KEY_DOT,
            KeyCode::KEY_SLASH,
            KeyCode::KEY_LEFTALT,
            KeyCode::KEY_RIGHTALT,
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_RIGHTSHIFT,
            KeyCode::KEY_LEFTCTRL,
            KeyCode::KEY_RIGHTCTRL,
            KeyCode::KEY_UP,
            KeyCode::KEY_DOWN,
        ]
        .iter()
        .filter(|k| s.contains(**k))
        .collect::<Vec<_>>();
        dbg!(supported_keys);
    }

    let mut play_machine = PlayMachine::new(ctx, main_args.a_frequency);
    loop {
        for event in keyboard
            .fetch_events()
            .expect("Cannot fetch the next keyboard event")
        {
            use evdev::EventSummary::*;
            let Key(_ev, key_code, val) = event.destructure() else {
                continue;
            };
            if main_args.debug {
                dbg!(key_code, val);
            }
            if val == 0 {
                play_machine.handle(false, key_code);
            } else {
                play_machine.handle(true, key_code);
            }
        }
    }
}

struct PlayMachine {
    player: NotePlayer,
    key_state: KeyState,
}

#[derive(Clone, Copy, Debug, Default)]
struct KeyState {
    note_key: Option<Note>,
    modifiers: u8,
}

impl KeyState {
    const S: u8 = 0b1;
    const FLAT: u8 = 0b10;
    const SHARP: u8 = 0b100;
    const OTTAVA_ALTA: u8 = 0b1000;
    const OTTAVA_BASSA: u8 = 0b10000;
}

impl TryFrom<KeyCode> for Note {
    type Error = KeyCode;
    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        use Note::*;
        match value {
            KeyCode::KEY_C => Ok(C),
            KeyCode::KEY_D => Ok(D),
            KeyCode::KEY_E => Ok(E),
            KeyCode::KEY_F => Ok(F),
            KeyCode::KEY_G => Ok(G),
            KeyCode::KEY_A => Ok(A),
            KeyCode::KEY_B => Ok(B),
            other => Err(other),
        }
    }
}

impl PlayMachine {
    fn new(ctx: AudioContext, a_frequency: f32) -> Self {
        Self {
            player: NotePlayer::new(ctx, a_frequency, 0, 0),
            key_state: Default::default(),
        }
    }

    pub fn handle(&mut self, is_on: bool, k: KeyCode) {
        let Self {
            player,
            key_state:
                KeyState {
                    note_key,
                    modifiers,
                },
        } = self;

        if is_on {
            match k {
                KeyCode::KEY_S | KeyCode::KEY_SPACE => {
                    *modifiers |= KeyState::S;
                }
                KeyCode::KEY_DOT => {
                    *modifiers |= KeyState::FLAT;
                }
                KeyCode::KEY_SLASH => {
                    *modifiers |= KeyState::SHARP;
                }
                KeyCode::KEY_LEFTALT | KeyCode::KEY_RIGHTALT => {
                    *modifiers |= KeyState::OTTAVA_BASSA;
                }
                KeyCode::KEY_LEFTSHIFT | KeyCode::KEY_RIGHTSHIFT => {
                    *modifiers |= KeyState::OTTAVA_ALTA;
                }
                KeyCode::KEY_UP => {
                    player.handle_transpose(0, 1);
                }
                KeyCode::KEY_DOWN => {
                    player.handle_transpose(0, -1);
                }
                k if let Ok(note) = k.try_into() => {
                    *note_key = Some(note);
                }
                _ => (),
            }
        } else {
            match k {
                KeyCode::KEY_DOT => {
                    *modifiers &= !KeyState::FLAT;
                }
                KeyCode::KEY_SLASH => {
                    *modifiers &= !KeyState::SHARP;
                }
                KeyCode::KEY_LEFTALT | KeyCode::KEY_RIGHTALT => {
                    *modifiers &= !KeyState::OTTAVA_BASSA;
                }
                KeyCode::KEY_LEFTSHIFT | KeyCode::KEY_RIGHTSHIFT => {
                    *modifiers &= !KeyState::OTTAVA_ALTA;
                }
                KeyCode::KEY_S | KeyCode::KEY_SPACE => {
                    *modifiers &= !KeyState::S;
                    if note_key.is_none() {
                        player.handle_stop()
                    }
                    return;
                }
                k => {
                    let Ok(note) = Note::try_from(k) else { return };
                    if Some(note) != *note_key {
                        return;
                    }
                    *note_key = None;
                    if (*modifiers & KeyState::S) != 0 {
                        return;
                    }
                    player.handle_stop();
                    return;
                }
            }
        }

        let Some(note) = note_key else {
            return;
        };
        let modif_has = |m| ((*modifiers & m) != 0) as isize;
        player.handle_play(
            *note,
            modif_has(KeyState::SHARP) - modif_has(KeyState::FLAT),
            modif_has(KeyState::OTTAVA_ALTA)
                - modif_has(KeyState::OTTAVA_BASSA),
        );
    }
}
