use web_audio_api::{
    context::{AudioContext, BaseAudioContext},
    node::{
        AudioNode, AudioNodeOptions, AudioScheduledSourceNode,
        OscillatorNode, OscillatorOptions, OscillatorType,
    },
};

struct PlayingStatus {
    osci: OscillatorNode,
}

struct TonePlayer {
    ctx: AudioContext,
    playing_status: Option<PlayingStatus>,
}

fn alter2(detune: f32, semitune: isize, octave: isize) -> f32 {
    detune + (100 * semitune) as f32 + (1200 * octave) as f32
}

impl TonePlayer {
    fn new(ctx: AudioContext) -> Self {
        Self {
            ctx,
            playing_status: None,
        }
    }

    fn start(&mut self, fundamental_frenquncy: f32, detune: f32) -> bool {
        if let Some(s) = &self.playing_status
            && s.osci.detune().value() == detune
        {
            return false;
        }
        self.stop();
        let mut osci = OscillatorNode::new(
            &self.ctx,
            OscillatorOptions {
                type_: OscillatorType::Sine,
                frequency: fundamental_frenquncy,
                detune,
                periodic_wave: None,
                audio_node_options: AudioNodeOptions::default(),
            },
        );
        osci.connect(&self.ctx.destination());
        osci.start();
        self.playing_status = Some(PlayingStatus { osci });
        true
    }

    fn stop(&mut self) {
        if let Some(s) = &mut self.playing_status {
            s.osci.stop();
            self.playing_status = None;
        }
    }
}

pub struct NotePlayer {
    player: TonePlayer,
    fundamental_frequency: f32,
    offset_semitone: isize,
    offset_octave: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl TryFrom<char> for Note {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Note::*;
        match value.to_ascii_lowercase() {
            'c' => Ok(C),
            'd' => Ok(D),
            'e' => Ok(E),
            'f' => Ok(F),
            'g' => Ok(G),
            'a' => Ok(A),
            'b' => Ok(B),
            other => Err(other),
        }
    }
}

impl Note {
    pub const fn detune_from_a(&self) -> f32 {
        use Note::*;
        match self {
            C => -900.,
            D => -700.,
            E => -500.,
            F => -400.,
            G => -200.,
            A => 0.,
            B => 200.,
        }
    }
}

impl NotePlayer {
    pub fn new(
        ctx: AudioContext,
        fundamental_frequency: f32,
        offset_semitone: isize,
        offset_octave: isize,
    ) -> Self {
        Self {
            player: TonePlayer::new(ctx),
            fundamental_frequency,
            offset_semitone,
            offset_octave,
        }
    }

    pub fn handle_transpose(&mut self, semitone: isize, octave: isize) {
        self.offset_octave += octave;
        self.offset_semitone += semitone;
        if let Some(s) = &self.player.playing_status {
            self.player.start(
                s.osci.frequency().value(),
                alter2(s.osci.detune().value(), semitone, octave),
            );
        }
    }

    pub fn handle_play(
        &mut self,
        note: Note,
        extra_semitone: isize,
        extra_octave: isize,
    ) {
        self.player.start(
            self.fundamental_frequency,
            alter2(
                note.detune_from_a(),
                extra_semitone + self.offset_semitone,
                extra_octave + self.offset_octave,
            ),
        );
    }

    pub fn handle_stop(&mut self) {
        self.player.stop()
    }
}
