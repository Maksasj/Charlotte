//! The Charlotte engine: default implementations of the layers, the turn pipeline, and learning.

pub mod connectors;
pub mod decide;
pub mod engine;
pub mod exchange_types;
pub mod grow;
pub mod learn;
pub mod numwords;
pub mod realize;
pub mod recall;
pub mod regress;
pub mod sim;
pub mod track;
pub mod understand;
pub mod view;

pub use learn::{Corpus, LearnOptions, Report};
pub use engine::{default_layers, ConvState, Conversation, Engine, Item, ItemSummary, LayerFactory, Layers, Message, TrustChange, Verdict};
