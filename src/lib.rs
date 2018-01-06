#![warn(missing_docs)]
#![deny(missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]

//!
//! # mpris
//!
//! `mpris` is an idiomatic library for dealing with MPRIS2-compatible media players over D-Bus.
//!
//! This would mostly apply to the Linux-ecosystem which is a heavy user of D-Bus.
//!
//! ## Getting started
//!
//! Some hints on how to use this library:
//!
//! 1. Look at the examples under `examples/`.
//! 2. Look at the `PlayerFinder` struct.
//!

// Rust currently has a false-positive on unused_imports for proc macro crates:
// If it's imported with #[macro_use] it triggers the "Unused imports" lint.
// If you remove #[macro_use], then the custom derives stop working with a recommendation to add it
// again.
//
// Allowing unused_imports on this statement gets rid of the warning.
#[allow(unused_imports)]
#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate failure;

extern crate dbus;

mod generated;
mod extensions;

mod pooled_connection;
mod find;
mod metadata;
mod player;
mod progress;

pub use find::{FindingError, PlayerFinder};
pub use metadata::Metadata;
pub use player::Player;
pub use progress::{Progress, ProgressTracker};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(missing_docs)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// A Player's looping status.
///
/// See: [MPRIS2 specification about
/// `Loop_Status`](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html#Enum:Loop_Status)
pub enum LoopStatus {
    /// The playback will stop when there are no more tracks to play
    None,

    /// The current track will start again from the begining once it has finished playing
    Track,

    /// The playback loops through a list of tracks
    Playlist,
}

/// `PlaybackStatus` had an invalid string value.
#[derive(Fail, Debug)]
#[fail(display = "PlaybackStatus must be one of Playing, Paused, Stopped, but was {}", _0)]
pub struct InvalidPlaybackStatus(String);

impl ::std::str::FromStr for PlaybackStatus {
    type Err = InvalidPlaybackStatus;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use PlaybackStatus::*;

        match string {
            "Playing" => Ok(Playing),
            "Paused" => Ok(Paused),
            "Stopped" => Ok(Stopped),
            other => Err(InvalidPlaybackStatus(other.to_string())),
        }
    }
}

/// `LoopStatus` had an invalid string value.
#[derive(Fail, Debug)]
#[fail(display = "LoopStatus must be one of None, Track, Playlist, but was {}", _0)]
pub struct InvalidLoopStatus(String);

impl ::std::str::FromStr for LoopStatus {
    type Err = InvalidLoopStatus;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "None" => Ok(LoopStatus::None),
            "Track" => Ok(LoopStatus::Track),
            "Playlist" => Ok(LoopStatus::Playlist),
            other => Err(InvalidLoopStatus(other.to_string())),
        }
    }
}

impl LoopStatus {
    fn dbus_value(&self) -> String {
        String::from(match *self {
            LoopStatus::None => "None",
            LoopStatus::Track => "Track",
            LoopStatus::Playlist => "Playlist",
        })
    }
}

/// Something went wrong when communicating with the D-Bus. This could either be an underlying
/// D-Bus library problem, or that the other side did not conform to the expected protocols.
#[derive(Fail, Debug)]
#[fail(display = "D-Bus call failed: {}", message)]
pub struct DBusError {
    /// The reported error message from the underlying D-Bus error.
    message: String,
}

impl DBusError {
    fn new<S: Into<String>>(message: S) -> Self {
        DBusError {
            message: message.into(),
        }
    }
}

impl From<dbus::Error> for DBusError {
    fn from(error: dbus::Error) -> Self {
        DBusError {
            message: error
                .message()
                .unwrap_or("No error message present")
                .to_string(),
        }
    }
}

impl From<InvalidPlaybackStatus> for DBusError {
    fn from(error: InvalidPlaybackStatus) -> Self {
        DBusError {
            message: error.to_string(),
        }
    }
}

impl From<InvalidLoopStatus> for DBusError {
    fn from(error: InvalidLoopStatus) -> Self {
        DBusError {
            message: error.to_string(),
        }
    }
}
