//! # Minimal Timer (`mtimer`)

use argh::FromArgs;
use mtimer::{ImportPair, SoundManager, Timer, errors::TimerError, parse_file};

pub const PLANS_FOLDER:   &str = "plans/";
pub const DEFAULT_START:  &str = "default/start.wav";
pub const DEFAULT_STOP:   &str = "default/stop.wav";
pub const DEFAULT_END:    &str = "default/go.wav";
pub const DEFAULT_1SEC:   &str = "default/1_second.wav";
pub const DEFAULT_2SECS:  &str = "default/2_seconds.wav";
pub const DEFAULT_3SECS:  &str = "default/3_seconds.wav";
pub const DEFAULT_4SECS:  &str = "default/4_seconds.wav";
pub const DEFAULT_5SECS:  &str = "default/5_seconds.wav";
pub const DEFAULT_10SECS: &str = "default/10_seconds.wav";

/// Top-level command.
#[derive(FromArgs, PartialEq, Debug)]
struct Cli {
    #[argh(subcommand)]
    subcommands: SubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    New(New),
    Plan(Plan),
}

/// Create a timer of given `length` and optional `volume` and `countdown`.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "time")]
struct New {
    /// length of the timer - mandatory.
    #[argh(positional)]
    length: u32,
    /// volume of the sounds, from 0 to 100.
    #[argh(option, short = 'v', default = "default_volume()")]
    volume: u8,     
    /// use a countdown (e.g. 3, 2, 1).  By default, sound played at end only.
    #[argh(switch, short = 'c')]
    countdown: bool,       
}

/// Create a Timer from a `.txt` timer plan.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "plan")]
struct Plan {
    /// name of the timer plan (no .txt needed) - mandatory.
    #[argh(positional)]    
    name: String,
    /// volume of the sounds, from 0 to 200%.
    #[argh(option, short = 'v', default = "default_volume()")]
    volume: u8,      
}

/// Sets default volume for the Timer.  Volume ranges from 0 to 100.
fn default_volume() -> u8 {
    50
}

/// Runs either `basic` or `plan` functionality based on CLI arguments.
///
/// `volume` is converted from 0 to 200 `u8` to a 0.0 to 2.0 `f32` for use by `rodio`.
fn run_cli(cli: &Cli) -> Result<(), TimerError> {
    match &cli.subcommands {
        SubCommands::New(b) => {
            let length = b.length as u64;
            let volume = b.volume.max(0).min(100) as f32 / 100_f32;
            let countdown = b.countdown;
            run_timer_basic(length, volume, countdown)
        }
        SubCommands::Plan(p) => {
            let name = &p.name;
            let volume = p.volume.max(0).min(100) as f32 / 100_f32;
            run_timer_plan(&name, volume)
        }
    }
}

/// Runs the `Timer` according to the `basic` CLI subcommand.
///
/// If `countdown` is set, default countdown sounds will play at the following intervals,
/// given that the `length` exceeds said interval:
/// - 10 seconds
/// -  5 seconds
/// -  3 seconds
fn run_timer_basic(length: u64, volume: f32, countdown: bool) -> Result<(), TimerError> {
    let mut sound_mgr = SoundManager::new(volume)?;
        
    if countdown {        
        if length > 10 {
            let start = ImportPair::new(DEFAULT_START, length - 10);
            let ten = ImportPair::new(DEFAULT_10SECS, 5);
            let five = ImportPair::new(DEFAULT_5SECS, 1);
            let four = ImportPair::new(DEFAULT_4SECS, 1);            
            let three = ImportPair::new(DEFAULT_3SECS, 1);
            let two = ImportPair::new(DEFAULT_2SECS, 1);
            let one = ImportPair::new(DEFAULT_1SEC, 1);
            let end = ImportPair::new(DEFAULT_END, 1);        
            let timer = Timer::new(
                &[start, ten, five, four, three, two, one, end],
                &mut sound_mgr
            );
            return timer.run(&mut sound_mgr);
        } else if length > 5 {
            let start = ImportPair::new(DEFAULT_START, length - 5);
            let five = ImportPair::new(DEFAULT_5SECS, 1);
            let four = ImportPair::new(DEFAULT_4SECS, 1);
            let three = ImportPair::new(DEFAULT_3SECS, 1);
            let two = ImportPair::new(DEFAULT_2SECS, 1);
            let one = ImportPair::new(DEFAULT_1SEC, 1);
            let end = ImportPair::new(DEFAULT_END, 1);        
            let timer = Timer::new(
                &[start, five, four, three, two, one, end],
                &mut sound_mgr
            );
            return timer.run(&mut sound_mgr);            
        } else if length > 3 {
            let start = ImportPair::new(DEFAULT_START, length - 3);
            let three = ImportPair::new(DEFAULT_3SECS, 1);
            let two = ImportPair::new(DEFAULT_2SECS, 1);
            let one = ImportPair::new(DEFAULT_1SEC, 1);
            let end = ImportPair::new(DEFAULT_END, 1);        
            let timer = Timer::new(
                &[start, three, two, one, end], 
                &mut sound_mgr
            );
            return timer.run(&mut sound_mgr);            
        }        
    }

    let start_pair = ImportPair::new(DEFAULT_START, length);
    let end_pair = ImportPair::new(DEFAULT_END, 1);        
    let timer = Timer::new(
        &[start_pair, end_pair], &mut sound_mgr
    );
    timer.run(&mut sound_mgr)
}

/// Runs the `Timer` according to the `plan` CLI subcommand.
fn run_timer_plan(name: &str, volume: f32) -> Result<(), TimerError> {
    let mut sound_mgr = SoundManager::new(volume)?;
    let fp = format!("{}/{}.txt", PLANS_FOLDER, name);
    let pairs = parse_file(&fp)?;
    let timer = Timer::new(&pairs, &mut sound_mgr);
    timer.run(&mut sound_mgr)
}

fn main() -> Result<(), TimerError> {
    let cli: Cli = argh::from_env();
    run_cli(&cli)?;

    Ok(())
}
