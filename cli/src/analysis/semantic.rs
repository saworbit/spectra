#[cfg(feature = "semantic")]
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use std::path::Path;

#[cfg(feature = "semantic")]
use std::fs::File;
#[cfg(feature = "semantic")]
use std::io::Read;

#[derive(Debug, Default, Clone)]
pub struct ContentTags {
    pub category: String, // e.g., "Contract", "Code", "Invoice"
    pub confidence: f64,
}

pub struct SemanticEngine {
    #[cfg(feature = "semantic")]
    model: Option<ZeroShotClassificationModel>,
}

impl SemanticEngine {
    pub fn new() -> Self {
        #[cfg(feature = "semantic")]
        {
            println!("🧠 Loading Neural Engine (DistilBERT)...");
            // In production, handle errors gracefully and maybe lazy load
            let model = ZeroShotClassificationModel::new(Default::default()).ok();
            if model.is_none() {
                eprintln!("⚠️  Warning: Failed to load ML model. Semantic analysis disabled.");
            }
            return Self { model };
        }

        #[cfg(not(feature = "semantic"))]
        Self {}
    }

    pub fn classify(&self, path: &Path) -> Option<ContentTags> {
        #[cfg(not(feature = "semantic"))]
        {
            let _ = path; // Suppress unused variable warning
            None
        }

        #[cfg(feature = "semantic")]
        {
            let model = self.model.as_ref()?;

            // 1. Read Sample
            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(_) => return None,
            };
            let mut buffer = [0u8; 2048]; // Small sample for text classification
            let n = file.read(&mut buffer).unwrap_or(0);
            if n == 0 {
                return None;
            }

            // 2. Decode (Lossy to handle binary/text mix)
            let text_sample = String::from_utf8_lossy(&buffer[..n]);

            // Skip if the sample is mostly non-text (binary)
            if text_sample
                .chars()
                .filter(|c| c.is_control() && *c != '\n' && *c != '\r' && *c != '\t')
                .count()
                > text_sample.len() / 10
            {
                return None;
            }

            // 3. Define Candidate Labels
            let candidate_labels = vec![
                "legal contract",
                "source code",
                "financial invoice",
                "personal letter",
                "log file",
                "configuration file",
                "documentation",
            ];

            // 4. Predict
            match model.predict(&[text_sample.as_ref()], &candidate_labels, None, 128) {
                Ok(predictions) => {
                    if let Some(result) = predictions.first() {
                        // The result structure from rust-bert contains labels with scores
                        // We take the highest scoring label
                        return Some(ContentTags {
                            category: result.text.clone(),
                            confidence: result.score as f64,
                        });
                    }
                    None
                }
                Err(_) => None,
            }
        }
    }
}

impl Default for SemanticEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_engine_creation() {
        // Should not panic regardless of feature flag
        let _engine = SemanticEngine::new();
    }

    #[test]
    fn test_classify_without_semantic_feature() {
        #[cfg(not(feature = "semantic"))]
        {
            let engine = SemanticEngine::new();
            let result = engine.classify(Path::new("test.txt"));
            assert!(result.is_none());
        }
    }
}
