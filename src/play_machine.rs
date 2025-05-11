use crate::tune_player::alter;

use crate::tune_player::detunes_from_c;
use crate::tune_player::ottava;

use crossterm::event::KeyModifiers;

use crossterm::event::KeyEvent;

use crate::tune_player::TunePlayer;

pub(crate) struct PlayMachine {
    pub(crate) player: TunePlayer,
    pub(crate) current_playing_char_lower: Option<char>,
    pub(crate) flat: bool,
    pub(crate) sharp: bool,
}

pub(crate) fn low_char2detune_from_c(ch: char) -> Option<f32> {
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

impl PlayMachine {
    pub(crate) fn new() -> Self {
        Self {
            player: TunePlayer::new(),
            current_playing_char_lower: None,
            sharp: false,
            flat: false,
        }
    }

    pub(crate) fn handle_on(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;
        if let Char(ch) = k.code {
            let ch = ch.to_ascii_lowercase();
            if let Some(detune) = low_char2detune_from_c(ch) {
                let modif = k.modifiers;
                let mut otta = 0;
                if modif.contains(KeyModifiers::SHIFT) {
                    otta += 1
                }
                if modif.contains(KeyModifiers::ALT) {
                    otta -= 1
                }
                let detune = ottava(detune, otta);
                let mut det = 0;
                if self.sharp {
                    det += 1
                }
                if self.flat {
                    det -= 1
                }
                let detune = alter(detune, det);
                self.player.start(detune);
                self.current_playing_char_lower = Some(ch);
            } else {
                match ch {
                    '.' | '>' => self.flat = true,
                    '/' | '?' => self.sharp = true,
                    _ => (),
                }
            }
        }
    }

    pub(crate) fn handle_off(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;
        if let Char(ch) = k.code {
            match ch {
                '.' | '>' => self.flat = false,
                '/' | '?' => self.sharp = false,
                x => {
                    let x = x.to_ascii_lowercase();
                    if let Some(cpc) = self.current_playing_char_lower {
                        if cpc == x {
                            self.player.stop();
                            self.current_playing_char_lower = None
                        }
                    }
                }
            }
        }
    }
}
