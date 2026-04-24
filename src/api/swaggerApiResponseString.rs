use serde::Deserialize;
use utoipa::ToSchema;

/// 统一的API响应格式
#[derive(Debug, Clone, ToSchema, Deserialize)]
pub struct ApiResponseString {
    /// 操作是否成功
    pub success: bool,
    /// 响应数据（仅当success为true时有效）
    pub data: Option<String>,
    /// 错误信息（仅当success为false时有效）
    pub error: Option<String>,
    /// 响应消息（用于显示给用户）
    pub message: String,
    /// 时间戳
    pub timestamp: i64,
}
