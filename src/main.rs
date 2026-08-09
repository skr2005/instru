use clap::{Parser, ValueEnum};
use web_audio_api::context::{
    AudioContext, AudioContextLatencyCategory, AudioContextOptions,
};

mod mode;
mod player;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct MainArgs {
    /// Enable logging and other things for debug use
    #[arg(long)]
    debug: bool,

    /// The frequency initially matching the key A
    ///
    /// Set it to 440 if you want the key C to initially match middle C.
    #[arg(long, short, default_value_t = 880.)]
    a_frequency: f32,

    /// A hint indicating tradeoffs among audio output latency,
    /// power consumption and glitches
    ///
    /// One can pass "balanced", "interactive", "playback"
    /// or a double-precision floating-point number to this argument.
    /// The meaning of each option is nearly the same as that of
    /// [the Web Audio API](https://developer.mozilla.org/docs/Web/API/AudioContext/AudioContext#latencyhint).
    /// However, under some circumstances, the default option "interactive" can cause glitches in the audio.
    /// To fix the problem, one may want to set this argument to "playback" or a larger number, or to compile this program with another backend.
    /// See the upstream project [web-audio-api-rs](https://github.com/orottier/web-audio-api-rs) for more details.
    #[arg(long, short, default_value = "interactive")]
    latency_hint: String,

    /// The mode to use
    #[arg(long, short, value_enum, default_value_t)]
    mode: MainMode,

    /// The path of input device used as the keyboard
    ///
    /// This argument will be ignored in modes other than "evdev".
    ///
    /// The program should have enough permission to access the path specified.
    ///
    /// The path is usually something like `/dev/input/event0`.
    /// Information printed in "evdev-helper" mode might be useful
    /// to choose the right path.
    #[cfg(unix)]
    #[arg(long, short)]
    keyboard: Option<std::path::PathBuf>,
}

#[derive(Default, ValueEnum, Clone, Debug, PartialEq, Eq)]
pub(crate) enum MainMode {
    /// Watch key pressing and releasing events in raw mode of the terminal
    #[default]
    Terminal,
    /// Watch keyboard events from a linux keyboard device
    #[cfg(unix)]
    Evdev,
    /// List informations about linux input devices and exit
    ///
    /// The program should have enough permission to access the devices.
    #[cfg(unix)]
    EvdevHelper,
}

impl MainArgs {
    fn audio_context(&self) -> AudioContext {
        use AudioContextLatencyCategory::*;
        let res = AudioContext::new(AudioContextOptions {
            latency_hint: match self.latency_hint.as_str() {
                "balanced" => Balanced,
                "interactive" => Interactive,
                "playback" => Playback,
                s if let Ok(f) = s.parse() => Custom(f),
                _ => panic!("The argument for latency hint is incorrect"),
            },
            ..Default::default()
        });

        if self.debug {
            dbg!(res.base_latency());
            dbg!(res.output_latency());
            dbg!(res.sink_id());
        }

        res
    }

    fn run_app(self) {
        use MainMode::*;
        match self.mode {
            Terminal => mode::terminal::play_loop(&self),
            #[cfg(unix)]
            Evdev => mode::evdev::play_loop(&self),
            #[cfg(unix)]
            EvdevHelper => mode::evdev_helper::main(&self),
        }
    }
}

fn main() {
    MainArgs::parse().run_app();
}
