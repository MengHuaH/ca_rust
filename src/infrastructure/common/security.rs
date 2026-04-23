use bcrypt::{DEFAULT_COST, hash, verify};
use tokio::task;
use tracing::error;

/// 密码安全工具
pub struct PasswordSecurity;

impl PasswordSecurity {
    /// 异步密码哈希（使用后台线程避免阻塞事件循环）
    pub async fn hash_password_async(password: String) -> Result<String, Box<dyn std::error::Error>> {
        // 使用 spawn_blocking 将 CPU 密集型任务移到后台线程
        let result = task::spawn_blocking(move || {
            hash(password, DEFAULT_COST)
        }).await;

        match result {
            Ok(hash_result) => {
                hash_result.map_err(|e| {
                    error!("密码哈希失败: {}", e);
                    Box::new(e) as Box<dyn std::error::Error>
                })
            }
            Err(e) => {
                error!("异步密码哈希任务失败: {}", e);
                Err(Box::new(e) as Box<dyn std::error::Error>)
            }
        }
    }

    /// 异步密码验证
    pub async fn verify_password_async(password: &str, hash: &str) -> bool {
        let password = password.to_string();
        let hash = hash.to_string();

        let result = task::spawn_blocking(move || {
            verify(password, &hash)
        }).await;

        match result {
            Ok(verify_result) => verify_result.unwrap_or(false),
            Err(e) => {
                error!("异步密码验证任务失败: {}", e);
                false
            }
        }
    }

    /// 快速密码强度验证（同步，用于快速失败）
    pub fn validate_password_strength(password: &str) -> Result<(), &'static str> {
        if password.len() < 6 {
            return Err("密码长度至少6位");
        }

        // 可以添加更多密码强度规则
        // 如：必须包含数字、字母、特殊字符等
        
        Ok(())
    }
}