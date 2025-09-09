use reqwest::{Client, RequestBuilder};
use super::store::USER_STORE;

/// 获取带有认证头的HTTP客户端
pub struct AuthenticatedClient {
    client: Client,
}

impl AuthenticatedClient {
    /// 创建新的认证客户端
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    
    /// 获取原始客户端（用于不需要认证的请求，如登录）
    pub fn raw_client(&self) -> &Client {
        &self.client
    }
    
    /// 获取带认证头的GET请求构建器
    #[allow(dead_code)]
    pub fn get(&self, url: &str) -> Result<RequestBuilder, String> {
        let mut request = self.client.get(url);
        
        if let Some(auth_header) = USER_STORE.get_current_auth_header() {
            request = request.header("Authorization", auth_header);
        } else {
            return Err("用户未登录或token已过期".to_string());
        }
        
        Ok(request)
    }
    
    /// 获取带认证头的POST请求构建器
    #[allow(dead_code)]
    pub fn post(&self, url: &str) -> Result<RequestBuilder, String> {
        let mut request = self.client.post(url);
        
        if let Some(auth_header) = USER_STORE.get_current_auth_header() {
            request = request.header("Authorization", auth_header);
        } else {
            return Err("用户未登录或token已过期".to_string());
        }
        
        Ok(request)
    }
    
    /// 获取带认证头的PUT请求构建器
    #[allow(dead_code)]
    pub fn put(&self, url: &str) -> Result<RequestBuilder, String> {
        let mut request = self.client.put(url);
        
        if let Some(auth_header) = USER_STORE.get_current_auth_header() {
            request = request.header("Authorization", auth_header);
        } else {
            return Err("用户未登录或token已过期".to_string());
        }
        
        Ok(request)
    }
    
    /// 获取带认证头的DELETE请求构建器
    #[allow(dead_code)]
    pub fn delete(&self, url: &str) -> Result<RequestBuilder, String> {
        let mut request = self.client.delete(url);
        
        if let Some(auth_header) = USER_STORE.get_current_auth_header() {
            request = request.header("Authorization", auth_header);
        } else {
            return Err("用户未登录或token已过期".to_string());
        }
        
        Ok(request)
    }
}

impl Default for AuthenticatedClient {
    fn default() -> Self {
        Self::new()
    }
}

// 全局认证客户端实例
lazy_static::lazy_static! {
    pub static ref AUTH_CLIENT: AuthenticatedClient = AuthenticatedClient::new();
}
