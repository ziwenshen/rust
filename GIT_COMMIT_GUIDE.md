# Git 提交指南

## 本次提交的主要更改

### 新增功能
1. **完整的用户认证系统**
   - 实现了用户登录、注销功能
   - 添加了用户会话存储和管理
   - 集成了HTTP客户端with自动认证头

### 模块结构
```
src-tauri/src/auth/
├── mod.rs      - 模块入口
├── types.rs    - 数据类型定义 (ApiResponse, LoginData等)
├── store.rs    - 用户会话存储 (UserSession, USER_STORE)
├── client.rs   - HTTP认证客户端 (AuthenticatedClient)
└── login.rs    - 登录相关Tauri命令
```

### Tauri命令
- `login(username, password)` - 用户登录
- `get_current_user()` - 获取当前用户信息
- `get_current_token()` - 获取认证token
- `is_logged_in()` - 检查登录状态
- `logout()` - 注销用户
- `minimize()` - 最小化窗口
- `close()` - 关闭窗口

### 技术特性
- Token自动过期检查
- 线程安全的用户状态管理
- 模块化代码架构
- 统一的HTTP客户端封装

## Git 提交步骤

在Git Bash中执行以下命令：

```bash
# 1. 检查当前状态
git status

# 2. 添加所有更改
git add .

# 3. 提交更改
git commit -m "feat: 实现完整的用户认证系统

- 添加用户登录、注销功能
- 实现用户会话存储和管理
- 创建HTTP认证客户端封装
- 添加Token自动过期检查
- 重构代码为模块化架构
- 新增Tauri命令接口：login, logout, get_current_user等"

# 4. 推送到远程仓库
git push origin main
```

## 文件清单

### 新增文件：
- `src-tauri/src/auth/mod.rs`
- `src-tauri/src/auth/types.rs`
- `src-tauri/src/auth/store.rs`
- `src-tauri/src/auth/client.rs`
- `src-tauri/src/auth/login.rs`
- `src-tauri/src/window.rs`

### 修改文件：
- `src-tauri/src/lib.rs` - 添加模块导入和命令注册
- `src-tauri/Cargo.toml` - 添加lazy_static依赖
- `src/components/login.rs` - 更新登录逻辑，使用新的API结构
- 其他相关UI文件

这个提交实现了一个完整的、生产就绪的用户认证系统。
