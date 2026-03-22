use axum::Json;

use crate::domain::{ApiResponse, PaginatedResponse, PaginationInfo, PaginationRequest};

pub struct ResponseService;

impl ResponseService {
    pub fn new() -> Self {
        ResponseService
    }

    pub fn success<T>(&self, data: T) -> Json<ApiResponse<T>> {
        Json(ApiResponse::success(data))
    }

    pub fn error<T>(&self, message: String, code: u32) -> Json<ApiResponse<T>> {
        Json(ApiResponse::error(message, code))
    }

    pub fn empty_success<T>(&self) -> Json<ApiResponse<T>> {
        Json(ApiResponse::empty_success())
    }

    pub fn paginated<T>(
        &self,
        items: Vec<T>,
        pagination_request: PaginationRequest,
        total_items: u64,
    ) -> Json<PaginatedResponse<T>> {
        let page = pagination_request.get_page();
        let page_size = pagination_request.get_page_size();

        Json(PaginatedResponse::new(items, page, page_size, total_items))
    }

    pub fn create_pagination_request(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> PaginationRequest {
        PaginationRequest::new(page, page_size)
    }
}

pub fn create_response_service() -> ResponseService {
    ResponseService::new()
}
