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

pub struct TunePlayer {
    ctx: AudioContext,
    playing_status: Option<PlayingStatus>,
}

pub fn ottava(detune: f32, n: isize) -> f32 {
    detune + (1200 * n) as f32
}

pub fn alter(detune: f32, n: isize) -> f32 {
    detune + (100 * n) as f32
}

impl Default for TunePlayer {
    fn default() -> Self {
        let ctx = AudioContext::default();
        Self {
            ctx,
            playing_status: None,
        }
    }
}

impl TunePlayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(
        &mut self,
        fundamental_frenquncy: f32,
        detune: f32,
    ) -> bool {
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

    pub fn stop(&mut self) {
        if let Some(s) = &mut self.playing_status {
            s.osci.stop();
            self.playing_status = None;
        }
    }
}
