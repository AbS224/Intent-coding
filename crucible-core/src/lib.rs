//! Crucible Engine Core - "Correct by Design, Not by Debugging"

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirement {
    pub id: Uuid,
    pub content: String,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAst {
    pub id: Uuid,
    pub requirements: Vec<Requirement>,
    pub correctness_score: f64,
}

impl IntentAst {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            requirements: Vec::new(),
            correctness_score: 0.0,
        }
    }

    pub fn add_requirement(&mut self, content: String) {
        let req = Requirement {
            id: Uuid::new_v4(),
            content,
            verified: false,
        };
        self.requirements.push(req);
        self.update_score();
    }

    fn update_score(&mut self) {
        if self.requirements.is_empty() {
            self.correctness_score = 0.0;
            return;
        }
        
        let verified = self.requirements.iter().filter(|r| r.verified).count();
        self.correctness_score = (verified as f64 / self.requirements.len() as f64) * 100.0;
    }
}

impl Default for IntentAst {
    fn default() -> Self {
        Self::new()
    }
}