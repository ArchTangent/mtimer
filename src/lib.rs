//! Library for `mtimer`.

pub mod errors;
mod string_store;

use errors::TimerError;
use rodio::{OutputStream, OutputStreamHandle};
use string_store::StringStore;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::{Duration, Instant};

type SoundHandle = u16;

const TICK_INCREMENT: Duration = Duration::from_millis(100);
const SOUND_FOLDER: &str = "sound/";

pub enum ParsedLine {
    Entry(ImportPair),  // A valid TimerPair
    Comment,            // A comment beginning with #
    Blank,              // A blank line
}

/// Reads a file into a list of `TimerPair` instances.
pub fn parse_file(fp: &str) -> Result<Vec<ImportPair>, TimerError> {
    let mut pairs = Vec::with_capacity(5);
    match File::open(fp) {
        Ok(f) => {
            let buffered = BufReader::new(f);
            for line in buffered.lines() {
                match line {
                    Ok(l) => {
                        let parsed_line = parse_line(&l)?;
                        if let ParsedLine::Entry(pair) = parsed_line {
                            pairs.push(pair);
                        }    
                    }
                    Err(_) => {
                        return Err(TimerError::InvalidFile { file: fp.into() });
                    }
                }                            
            }
        },
        Err(_) => {
            return Err(TimerError::InvalidFile { file: fp.into() });
        }
    }
    
    Ok(pairs)
}

/// Parses a line from a timer plan.
fn parse_line(s: &str) -> Result<ParsedLine, TimerError> {
    // Skip blank lines and comments (# as first character in line)
    if let Some(0) = s.find('#') { return Ok(ParsedLine::Comment); }
    if s.is_empty() { return Ok(ParsedLine::Blank); }
    // Split filepath and time delay (default 1) at ':'
    let mut tokens = s.trim().split(':');
    let filepath = match tokens.next() {
        Some(fp) => match fp.ends_with(".wav") {
            true => fp,
            false => return Err(TimerError::InvalidParse { line: s.into() }),
        }
        None => return Err(TimerError::InvalidParse { line: s.into() }),
    };    
    let delay = match tokens.next() {
        Some(d) => {
            match d.trim().parse::<u64>() {
                Ok(val) => val,
                Err(_) => return Err(TimerError::InvalidParse { line: s.into() }),
            }
        },
        None => 1_u64,
    };

    Ok(
        ParsedLine::Entry(
            ImportPair {
                sound_path: filepath.into(),
                time_delay: delay,
            }
        )
    )
}

/// Handles sound file paths and sound playback.
pub struct SoundManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sound_paths: StringStore,
    volume: f32,
}

impl SoundManager {
    pub fn new(volume: f32) -> Result<Self, TimerError> {
        // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (_stream, stream_handle) = OutputStream::try_default()?;

        Ok(Self {
            _stream,
            stream_handle,
            sound_paths: StringStore::with_capacity(5),
            volume,
        })
    }
    pub fn get_handle(&mut self, path: &str) -> Option<SoundHandle> {
        self.sound_paths.get_handle(path)
    }
    /// Plays sound from the given `SoundHandle`.
    pub fn play(&mut self, sound_handle: SoundHandle) -> Result<(), TimerError> {
        let mut sound_path = SOUND_FOLDER.to_string();
        sound_path.push_str(&self.sound_paths[sound_handle]);
        let file = std::fs::File::open(sound_path)?;
        let sound = self.stream_handle.play_once(BufReader::new(file))?;
        sound.set_volume(self.volume);
        sound.detach();

        Ok(())
    }
}

/// Top-level timer structure.  Holds all sounds to be played.
pub struct Timer {
    plan: Vec<TimerPair>,
}

impl Timer {
    /// Makes new Timer instance from list of `ImportPair` data.
    pub fn new(pairs: &[ImportPair], sound_mgr: &mut SoundManager) -> Self {
        let mut plan = Vec::with_capacity(5);
        
        for pair in pairs.iter() {
            // Get Sound handle from sound_paths
            let sound_handle = sound_mgr.get_handle(&pair.sound_path);
            let time_delay = Duration::from_secs(pair.time_delay);
            plan.push(TimerPair::new(sound_handle, time_delay));
        }

        Timer { 
            plan,
        }
    }   
    pub fn run(&self, sound_mgr: &mut SoundManager) -> Result<(), TimerError> {
        for (ix, timer_data) in self.plan.iter().enumerate() {
            let start = Instant::now();
            self.play(ix, timer_data.sound_handle, sound_mgr)?;
            while Instant::now() - start < timer_data.time_delay {
                thread::sleep(TICK_INCREMENT);
            }
        }

        Ok(())
    }    
    pub fn play(
        &self, ix: usize, 
        sound_handle: Option<SoundHandle>, 
        sound_mgr: &mut SoundManager
    ) -> Result<(), TimerError> {
        if let Some(sound_handle) = sound_handle {
            sound_mgr.play(sound_handle)?;
        } else {
            println!("No sound at index [{}]!", ix);
        }    
        
        Ok(())
    }
}

/// Individual timer entry holding a `u16` handle to the sound path (if any) and a time delay
/// (`Duration`) until the next TimerPair is chosen.
#[derive(Debug)]
pub struct TimerPair {
    sound_handle:   Option<SoundHandle>,
    time_delay:     Duration,
}

impl TimerPair {
    pub fn new(sound_handle: Option<SoundHandle>, time_delay: Duration) -> Self { 
        Self { sound_handle, time_delay } 
    }
}

/// Version of a `TimerPair` loaded from a `.txt` file.
#[derive(Debug)]
pub struct ImportPair {
    sound_path: String,
    time_delay: u64,
}

impl ImportPair {
    pub fn new(sound_path: &str, time_delay: u64) -> Self {
        Self {
            sound_path: sound_path.into(),
            time_delay,
        }
    }
}
