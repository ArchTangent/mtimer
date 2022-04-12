//! # Minimal Timer (`mtimer`)
//! Can be run via arguments (basic) or a file (timer plan).

use argh::FromArgs;
use std::time::{Duration, Instant};

type SoundHandle = u16;

/// Entry point to the CLI.
#[derive(FromArgs, PartialEq, Debug)]
struct Cli {
    #[argh(subcommand)]
    subcommands: SubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    Basic(Basic),
}

/// Create a timer of given `length` and optional `volume` and `countdown`.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "time")]
struct Basic {
    /// length of the timer - mandatory.
    #[argh(positional)]
    length: u32,
    /// volume of the sounds, from 0 to 100.
    #[argh(option, short = 'v', default = "default_volume()")]
    volume: u8,
}

/// Sets default volume for the Timer.  Volume ranges from 0 to 100.
fn default_volume() -> u8 {
    50
}

/// Runs `basic` or `plan` functionality based on CLI arguments.
///
/// `volume` ranges from 0-200, scaled to a 0.0-2.0 `f32` for use with `rodio`.
fn run_cli(cli: &Cli) {
    match &cli.subcommands {
        SubCommands::Basic(b) => {
            let length = b.length as u64;
            let volume = b.volume.max(0).min(100) as f32 / 100_f32;
            run_timer_basic(length, volume)
        }
    }
}

/// Top-level timer structure.  Holds all sounds to be played.
pub struct Timer {
    plan: Vec<TimerPair>,
}

impl Timer {
    /// Makes new Timer instance.
    // TODO: tidy up with real sounds
    pub fn new(pairs: &[TimerPair]) -> Self {
        let mut plan = Vec::with_capacity(5);

        for pair in pairs.iter() {
            plan.push(pair.clone());
        }

        Timer { plan }
    }
    pub fn run(&self) {
        println!("Timer is running!");
    }
    pub fn play(&self, ix: usize, sound_handle: Option<SoundHandle>) {
        println!("Timer at ix {ix} and handle {sound_handle:#?} is playing!");
    }
}

/// Holds sound handle and time delay until the next TimerData is selected.
#[derive(Debug, Clone)]
pub struct TimerPair {
    sound_handle: Option<SoundHandle>,
    time_delay: Duration,
}

impl TimerPair {
    pub fn new(sound_handle: Option<SoundHandle>, time_delay: Duration) -> Self {
        Self {
            sound_handle,
            time_delay,
        }
    }
}

/// Runs the `Timer` according to the `basic` CLI subcommand.
fn run_timer_basic(length: u64, volume: f32) {
    let start_pair = TimerPair::new(Some(0), Duration::from_secs(length));
    let end_pair = TimerPair::new(Some(1), Duration::from_secs(length));
    let timer = Timer::new(&[start_pair, end_pair]);
    timer.run();
}

fn main() {
    println!("--- mtimer ---");
    let cli: Cli = argh::from_env();
    run_cli(&cli);
}
