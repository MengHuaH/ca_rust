use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::entities::{AnalysisResult, Image};

pub trait ImageRepository {
    fn save_image(&self, image: Image) -> Result<(), String>;
    fn get_image(&self, image_id: &str) -> Option<Image>;
    fn delete_image(&self, image_id: &str) -> Result<(), String>;
}

pub trait AnalysisRepository {
    fn save_analysis(&self, analysis: AnalysisResult) -> Result<(), String>;
    fn get_analysis(&self, image_id: &str) -> Option<AnalysisResult>;
}

#[derive(Clone)]
pub struct InMemoryImageRepository {
    images: Arc<RwLock<HashMap<String, Image>>>,
}

impl InMemoryImageRepository {
    pub fn new() -> Self {
        Self {
            images: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ImageRepository for InMemoryImageRepository {
    fn save_image(&self, image: Image) -> Result<(), String> {
        let mut images = self.images.blocking_write();
        images.insert(image.id.clone(), image);
        Ok(())
    }

    fn get_image(&self, image_id: &str) -> Option<Image> {
        let images = self.images.blocking_read();
        images.get(image_id).cloned()
    }

    fn delete_image(&self, image_id: &str) -> Result<(), String> {
        let mut images = self.images.blocking_write();
        if images.remove(image_id).is_some() {
            Ok(())
        } else {
            Err("Image not found".to_string())
        }
    }
}

#[derive(Clone)]
pub struct InMemoryAnalysisRepository {
    analyses: Arc<RwLock<HashMap<String, AnalysisResult>>>,
}

impl InMemoryAnalysisRepository {
    pub fn new() -> Self {
        Self {
            analyses: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl AnalysisRepository for InMemoryAnalysisRepository {
    fn save_analysis(&self, analysis: AnalysisResult) -> Result<(), String> {
        let mut analyses = self.analyses.blocking_write();
        analyses.insert(analysis.image_id.clone(), analysis);
        Ok(())
    }

    fn get_analysis(&self, image_id: &str) -> Option<AnalysisResult> {
        let analyses = self.analyses.blocking_read();
        analyses.get(image_id).cloned()
    }
}
