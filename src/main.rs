use clap::Parser;

use crate::mode::terminal;

mod mode;
mod play_machine;
mod tune_player;

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
}

fn main() {
    let args = MainArgs::parse();

    terminal::play_loop(&args);
}
