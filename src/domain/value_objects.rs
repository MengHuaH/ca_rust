use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageId(String);

impl ImageId {
    pub fn new(id: String) -> Result<Self, String> {
        if id.is_empty() {
            return Err("Image ID cannot be empty".to_string());
        }
        Ok(ImageId(id))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Confidence(f32);

impl Confidence {
    pub fn new(value: f32) -> Result<Self, String> {
        if value < 0.0 || value > 1.0 {
            return Err("Confidence must be between 0.0 and 1.0".to_string());
        }
        Ok(Confidence(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingTime(u64);

impl ProcessingTime {
    pub fn new(milliseconds: u64) -> Self {
        ProcessingTime(milliseconds)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}
