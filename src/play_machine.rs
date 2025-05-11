use crate::tune_player::alter;

use crate::tune_player::detunes_from_c;
use crate::tune_player::ottava;

use crossterm::event::KeyModifiers;

use crossterm::event::KeyEvent;

use crate::tune_player::TunePlayer;

pub struct PlayMachine {
    player: TunePlayer,
    current_playing_event: Option<KeyEvent>,
    space_on: Option<KeyEvent>,
    flat: bool,
    sharp: bool,
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

impl PlayMachine {
    pub(crate) fn new() -> Self {
        Self {
            player: TunePlayer::new(),
            current_playing_event: None,
            space_on: None,
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
                self.current_playing_event = Some(k);
            } else {
                let mut rep = true;
                match ch {
                    '.' | '>' => self.flat = true,
                    '/' | '?' => self.sharp = true,
                    ' ' => self.space_on = Some(k),
                    _ => rep = false,
                }
                if rep {
                    if let Some(e) = self.current_playing_event {
                        self.handle_on(e);
                    }
                }
            }
        }
    }

    pub(crate) fn handle_off(&mut self, k: KeyEvent) {
        use crossterm::event::KeyCode::*;
        if let Char(ch) = k.code {
            let mut rep = true;
            match ch {
                '.' | '>' => self.flat = false,
                '/' | '?' => self.sharp = false,
                ' ' => {
                    self.space_on = None;
                    if self.current_playing_event == None {
                        self.player.stop();
                    }
                }
                x => {
                    rep = false;
                    let x = x.to_ascii_lowercase();
                    if let Some(e) = self.current_playing_event {
                        if let Char(ch) = e.code {
                            let ch = ch.to_ascii_lowercase();
                            if ch == x {
                                self.current_playing_event = None;
                                if self.space_on.is_none() {
                                    self.player.stop();
                                }
                            }
                        }
                    }
                }
            }
            if rep {
                if let Some(e) = self.current_playing_event {
                    self.handle_on(e);
                }
            }
        }
    }
}
