#![doc = include_str!("../README.md")]

mod cache;
pub mod item;
pub mod replacement_policy;
pub mod stats;
pub mod trace;

pub use cache::Cache;
pub use item::{GeneralModelGenerator, GeneralModelItem};
pub use trace::Trace;

pub use replacement_policy::{Fifo, Landlord, Lfu, Lru, Mru, Rand};
