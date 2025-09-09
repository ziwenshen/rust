use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::types::LoginData;

/// 用户会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: u32,
    pub username: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub login_time: u64, // Unix时间戳
}

impl UserSession {
    /// 从登录数据创建用户会话
    pub fn from_login_data(login_data: LoginData) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            user_id: login_data.user_id,
            username: login_data.username,
            access_token: login_data.access_token,
            token_type: login_data.token_type,
            expires_in: login_data.expires_in,
            login_time: now,
        }
    }
    
    /// 检查token是否过期
    pub fn is_token_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        now > (self.login_time + self.expires_in as u64)
    }
    
    /// 获取Authorization header值
    pub fn get_auth_header(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}

/// 全局用户状态管理器
pub struct UserStore {
    current_session: Arc<Mutex<Option<UserSession>>>,
    // 可以扩展为支持多用户会话
    sessions: Arc<Mutex<HashMap<u32, UserSession>>>,
}

impl UserStore {
    /// 创建新的用户存储实例
    pub fn new() -> Self {
        Self {
            current_session: Arc::new(Mutex::new(None)),
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// 设置当前用户会话
    pub fn set_current_session(&self, session: UserSession) {
        let user_id = session.user_id;
        let username = session.username.clone(); // 提前克隆用户名
        
        // 存储到当前会话
        if let Ok(mut current) = self.current_session.lock() {
            *current = Some(session.clone());
        }
        
        // 存储到会话映射
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.insert(user_id, session);
        }
        
        println!("用户会话已保存: {}", username);
    }
    
    /// 获取当前用户会话
    pub fn get_current_session(&self) -> Option<UserSession> {
        if let Ok(session) = self.current_session.lock() {
            session.clone()
        } else {
            None
        }
    }
    
    /// 获取有效的当前用户会话（检查过期）
    pub fn get_valid_current_session(&self) -> Option<UserSession> {
        if let Some(session) = self.get_current_session() {
            if !session.is_token_expired() {
                Some(session)
            } else {
                println!("Token已过期，需要重新登录");
                self.clear_current_session();
                None
            }
        } else {
            None
        }
    }
    
    /// 清除当前用户会话
    pub fn clear_current_session(&self) {
        if let Ok(mut current) = self.current_session.lock() {
            if let Some(session) = current.take() {
                println!("用户会话已清除: {}", session.username);
            }
        }
    }
    
    /// 获取当前用户信息
    pub fn get_current_user_info(&self) -> Option<(String, u32)> {
        self.get_valid_current_session()
            .map(|session| (session.username, session.user_id))
    }
    
    /// 获取当前认证头
    pub fn get_current_auth_header(&self) -> Option<String> {
        self.get_valid_current_session()
            .map(|session| session.get_auth_header())
    }
}

impl Default for UserStore {
    fn default() -> Self {
        Self::new()
    }
}

// 全局用户存储实例
lazy_static::lazy_static! {
    pub static ref USER_STORE: UserStore = UserStore::new();
}
