use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: PaginatedData<T>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaginatedData<T> {
    pub items: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaginationInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaginationRequest {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn error(message: String, code: u32) -> Self {
        Self {
            success: false,
            code,
            message,
            data: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn empty_success() -> Self {
        Self {
            success: true,
            code: 200,
            message: "Success".to_string(),
            data: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    pub fn into_inner(self) -> ApiResponse<T> {
        self
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, page_size: u32, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (page_size as f64)).ceil() as u32;

        Self {
            success: true,
            code: 200,
            message: "Success".to_string(),
            data: PaginatedData {
                items,
                pagination: PaginationInfo {
                    page,
                    page_size,
                    total_items,
                    total_pages,
                    has_next: page < total_pages,
                    has_previous: page > 1,
                },
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl PaginationRequest {
    pub fn new(page: Option<u32>, page_size: Option<u32>) -> Self {
        Self {
            page: page.or(Some(1)),
            page_size: page_size.or(Some(20)),
        }
    }

    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn get_page_size(&self) -> u32 {
        self.page_size.unwrap_or(20).max(1).min(100)
    }

    pub fn get_offset(&self) -> u64 {
        ((self.get_page() - 1) * self.get_page_size()) as u64
    }
}
