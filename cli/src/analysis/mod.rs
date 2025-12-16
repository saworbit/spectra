/// Phase 2: The Semantic Bridge
///
/// This module provides tiered content analysis capabilities:
/// - Tier 0: Metadata (size, path, extension) - handled in main.rs
/// - Tier 1: Heuristics (entropy, filename patterns) - this module
/// - Tier 2: Semantic (AI-based content classification) - optional feature
///
/// All analysis is performed on file headers only (max 8KB) to maintain
/// the "zero-latency" performance characteristic of Spectra.
pub mod entropy;
pub mod heuristics;
pub mod semantic;

// Re-export commonly used types
pub use entropy::calculate_shannon_entropy;
pub use heuristics::{analyze_filename_risk, RiskLevel};
#[allow(unused_imports)] // Part of public API, used by external consumers
pub use semantic::{ContentTags, SemanticEngine};
