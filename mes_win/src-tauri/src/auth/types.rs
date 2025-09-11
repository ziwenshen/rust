use serde::{Deserialize, Serialize};

// 登录请求结构
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// 登录响应数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginData {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u32,
    pub username: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

// API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub code: u32,
    pub message: String,
    pub data: Option<LoginData>,
}

// 通用API响应结构（用于登出等不返回LoginData的接口）
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericApiResponse {
    pub code: u32,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub timestamp: String,
}

impl GenericApiResponse {
    pub fn success(&self) -> bool {
        self.code == 200
    }
}
