pub mod independence;
pub mod clayton;
pub mod frank;

// Re-export for easier imports
pub use independence::IndependenceCopula;
pub use clayton::ClaytonCopula;
pub use frank::FrankCopula;
