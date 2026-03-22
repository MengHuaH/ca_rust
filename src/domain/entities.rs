use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Bmp,
    Gif,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub image_id: String,
    pub features: Vec<Feature>,
    pub confidence: f32,
    pub processing_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub value: f32,
    pub coordinates: Option<(f32, f32)>,
}

impl Image {
    pub fn new(id: String, data: Vec<u8>, width: u32, height: u32, format: ImageFormat) -> Self {
        Self {
            id,
            data,
            width,
            height,
            format,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}