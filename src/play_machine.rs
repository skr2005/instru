use crate::tune_player::alter;

use crate::tune_player::ottava;

use crossterm::event::KeyModifiers;

use crossterm::event::KeyEvent;

use crate::tune_player::TunePlayer;

pub struct PlayMachine {
    player: TunePlayer,
    current_playing_event_normalized: Option<KeyEvent>,
    space_on: Option<KeyEvent>,
    flat: bool,
    sharp: bool,
    otta_from_default: isize,
    a_frequency: f32,
}

fn low_char2detune_from_c(ch: char) -> Option<f32> {
    use detunes_from_c::*;
    match ch.to_ascii_lowercase() {
        'c' => Some(C),
        'd' => Some(D),
        'e' => Some(E),
        'f' => Some(F),
        'g' => Some(G),
        'a' => Some(A),
        'b' => Some(B),
        _ => None,
    }
}

pub mod detunes_from_c {
    pub const C: f32 = -900.;
    pub const D: f32 = -700.;
    pub const E: f32 = -500.;
    pub const F: f32 = -400.;
    pub const G: f32 = -200.;
    pub const A: f32 = 0.;
    pub const B: f32 = 200.;
}

fn normalize_char_event(k: KeyEvent) -> KeyEvent {
    use crossterm::event::KeyCode::*;

    if let Char(ch) = k.code {
        KeyEvent {
            code: Char(ch.to_ascii_lowercase()),
            ..k
        }
    } else {
        k
    }
}

impl PlayMachine {
    pub(crate) fn new(a_frequency: f32) -> Self {
        Self {
            player: TunePlayer::new(),
            current_playing_event_normalized: None,
            space_on: None,
            sharp: false,
            flat: false,
            otta_from_default: 0,
            a_frequency,
        }
    }

    pub(crate) fn handle_on(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;

        let k = normalize_char_event(k);

        let handle_play = |me: &mut Self, detune| {
            let modif = k.modifiers;
            let mut otta = me.otta_from_default;
            if modif.contains(KeyModifiers::SHIFT) {
                otta += 1
            }
            if modif.contains(KeyModifiers::ALT) {
                otta -= 1
            }
            let detune = ottava(detune, otta);
            let mut det = 0;
            if me.sharp {
                det += 1
            }
            if me.flat {
                det -= 1
            }
            let detune = alter(detune, det);
            me.player.start(me.a_frequency, detune);
            me.current_playing_event_normalized = Some(k);
        };

        let handle_non_play = |me: &mut Self| {
            let rep = match k.code {
                Char('.') | Char('>') => {
                    me.flat = true;
                    true
                }
                Char('/') | Char('?') => {
                    me.sharp = true;
                    true
                }
                Char(' ') | Char('s') => {
                    me.space_on = Some(k);
                    true
                }
                Up => {
                    me.otta_from_default += 1;
                    true
                }
                Down => {
                    me.otta_from_default -= 1;
                    true
                }
                _ => false,
            };
            if rep && let Some(e) = me.current_playing_event_normalized {
                me.handle_on(e);
            }
        };

        if let Char(ch) = k.code
            && let Some(detune) = low_char2detune_from_c(ch)
        {
            handle_play(self, detune);
            return;
        }
        handle_non_play(self);
    }

    pub(crate) fn handle_off(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;

        let k = normalize_char_event(k);

        if let Char(ch) = k.code {
            let rep = match ch {
                '.' | '>' => {
                    self.flat = false;
                    true
                }
                '/' | '?' => {
                    self.sharp = false;
                    true
                }
                ' ' | 's' => {
                    self.space_on = None;
                    if self.current_playing_event_normalized.is_none() {
                        self.player.stop();
                    }
                    false
                }
                _ => {
                    if let Some(KeyEvent {
                        code: Char(ch_playing),
                        ..
                    }) = self.current_playing_event_normalized
                        && ch_playing == ch
                    {
                        self.current_playing_event_normalized = None;
                        if self.space_on.is_none() {
                            self.player.stop();
                        }
                    };
                    false
                }
            };
            if rep && let Some(e) = self.current_playing_event_normalized {
                self.handle_on(e);
            }
        }
    }
}
