//! The Charlotte engine: default implementations of the layers, the turn pipeline, and learning.

pub mod decide;
pub mod engine;
pub mod learn;
pub mod realize;
pub mod sim;
pub mod track;
pub mod understand;
pub mod view;

pub use learn::{Corpus, LearnOptions, Report};
pub use engine::{default_layers, ConvState, Conversation, Engine, Item, ItemSummary, LayerFactory, Layers, Message, TrustChange, Verdict};
