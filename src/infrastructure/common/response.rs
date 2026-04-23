use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 统一的API响应格式
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T = ()> {
    /// 操作是否成功
    pub success: bool,
    /// 响应数据（仅当success为true时有效）
    pub data: Option<T>,
    /// 错误信息（仅当success为false时有效）
    pub error: Option<ApiError>,
    /// 响应消息（用于显示给用户）
    pub message: String,
    /// 时间戳
    pub timestamp: i64,
}

/// 详细的错误信息
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    /// 错误代码
    pub code: String,
    /// 错误描述
    pub message: String,
    /// 详细错误信息（开发环境使用）
    pub details: Option<String>,
    /// 错误字段（用于表单验证错误）
    pub field_errors: Option<Vec<FieldError>>,
}

/// 字段级错误信息
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FieldError {
    /// 字段名
    pub field: String,
    /// 错误信息
    pub message: String,
    /// 错误代码
    pub code: String,
}

/// API响应工具
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// 创建成功响应（带数据）
    pub fn success<T: Serialize>(data: T, message: &str) -> (StatusCode, Json<ApiResponse<T>>) {
        let response = ApiResponse {
            success: true,
            data: Some(data),
            error: None,
            message: message.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        (StatusCode::OK, Json(response))
    }

    /// 创建成功响应（无数据）
    pub fn success_message(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        let response = ApiResponse {
            success: true,
            data: None,
            error: None,
            message: message.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        (StatusCode::OK, Json(response))
    }

    /// 创建错误响应
    pub fn error(
        status_code: StatusCode,
        error_code: &str,
        message: &str,
        details: Option<String>,
        field_errors: Option<Vec<FieldError>>,
    ) -> (StatusCode, Json<ApiResponse<()>>) {
        let error = ApiError {
            code: error_code.to_string(),
            message: message.to_string(),
            details,
            field_errors,
        };

        let response = ApiResponse {
            success: false,
            data: None,
            error: Some(error),
            message: message.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        (status_code, Json(response))
    }

    /// 创建验证错误响应
    pub fn validation_error(field_errors: Vec<FieldError>) -> (StatusCode, Json<ApiResponse<()>>) {
        Self::error(
            StatusCode::BAD_REQUEST,
            "VALIDATION_ERROR",
            "请求参数验证失败",
            None,
            Some(field_errors),
        )
    }

    /// 创建未找到错误响应
    pub fn not_found(resource: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        Self::error(
            StatusCode::NOT_FOUND,
            "NOT_FOUND",
            &format!("{}未找到", resource),
            None,
            None,
        )
    }

    /// 创建冲突错误响应
    pub fn conflict(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        Self::error(StatusCode::CONFLICT, "CONFLICT", message, None, None)
    }

    /// 创建服务器错误响应
    pub fn server_error(
        message: &str,
        details: Option<String>,
    ) -> (StatusCode, Json<ApiResponse<()>>) {
        Self::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR",
            message,
            details,
            None,
        )
    }

    /// 创建未授权错误响应
    pub fn unauthorized(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        Self::error(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            message,
            None,
            None,
        )
    }

    /// 创建禁止访问错误响应
    pub fn forbidden(message: &str) -> (StatusCode, Json<ApiResponse<()>>) {
        Self::error(StatusCode::FORBIDDEN, "FORBIDDEN", message, None, None)
    }
}

/// 常用的错误代码
pub mod error_codes {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const NOT_FOUND: &str = "NOT_FOUND";
    pub const CONFLICT: &str = "CONFLICT";
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const FORBIDDEN: &str = "FORBIDDEN";
    pub const INTERNAL_SERVER_ERROR: &str = "INTERNAL_SERVER_ERROR";
    pub const DATABASE_ERROR: &str = "DATABASE_ERROR";
    pub const NETWORK_ERROR: &str = "NETWORK_ERROR";
}
