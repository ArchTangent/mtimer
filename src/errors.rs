//! Custom errors for `mtimer`.

use std::fmt;

#[derive(Debug)]
pub enum TimerError {
    /// For invalid path to files.
    InvalidPath { path: String },
    /// For invalid or missing file name in given path.
    InvalidFile { file: String },
    /// For Parsing errors (bad line in file)
    InvalidParse { line: String },
    /// Invalid argument
    InvalidArgument { arg: String },
    /// IO Error
    Io { kind: std::io::ErrorKind },    
    /// Playback Error
    Playback { error: rodio::PlayError },   
    /// Stream Error
    Stream { error: rodio::StreamError },               
}

impl std::error::Error for TimerError {}

impl fmt::Display for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimerError::InvalidPath { path } => {
                write!(f, "Cannot find path '{}'", path)
            }
            TimerError::InvalidFile { file: filename } => {
                write!(f, "Error loading file '{}'", filename)
            }
            TimerError::InvalidParse { line } => {
                write!(f, "Invalid line '{}' in file: should be in 'filename.wav: 1' format", line)
            }
            TimerError::InvalidArgument { arg } => {
                write!(f, "Invalid argument for '{}'", arg)   
            }
            TimerError::Io { kind } => {
                write!(f, "IO Error: '{:?}'", kind)   
            }       
            TimerError::Playback { error } => {
                write!(f, "Playback Error: '{}'", error)   
            }  
            TimerError::Stream { error } => {
                write!(f, "Stream Error: '{}'", error)   
            }              
        }
    }
}

impl From<rodio::StreamError> for TimerError {
    fn from(other: rodio::StreamError) -> TimerError {
        TimerError::Stream { error: other } 
    }
}

impl From<rodio::PlayError> for TimerError {
    fn from(other: rodio::PlayError) -> TimerError {
        TimerError::Playback { error: other } 
    }
}

impl From<std::io::Error> for TimerError {
    fn from(other: std::io::Error) -> TimerError {
        TimerError::Io { kind: other.kind() } 
    }
}
