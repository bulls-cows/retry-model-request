# AGENTS.md

## Scope

- 本仓库默认语言: TypeScript (前端) + Rust (后端)
- 允许修改目录:
  - `src/` 目录下的所有 `.ts`、`.vue`、`.scss` 文件
  - `src-tauri/src/` 目录下的所有 `.rs` 文件
  - 配置文件（package.json, Cargo.toml, tauri.conf.json 等）
- 禁止修改目录: 无

## Quality Gate

- 改动后必须执行:
  - `npm run lint` (前端代码格式化、静态检查)
  - `cargo fmt --check` (Rust 代码格式检查)
  - `cargo clippy` (Rust 静态检查)

## Delivery Format

- 文件引用格式: `[文件名](file:///绝对路径#L行号)`
- 修改代码前先说明修改意图

## Project Structure

```
retry-model-request/
├── src-tauri/                    # Rust 后端 (Tauri)
│   ├── src/
│   │   ├── main.rs              # Tauri 入口
│   │   ├── lib.rs               # 库入口
│   │   ├── commands/            # Tauri Commands
│   │   ├── proxy/               # 代理服务核心
│   │   ├── config/              # 配置管理
│   │   ├── stats/               # 统计引擎
│   │   └── tray/                # 系统托盘
│   ├── Cargo.toml               # Rust 依赖配置
│   └── tauri.conf.json          # Tauri 配置
├── src/                          # Vue 3 前端
│   ├── components/              # 组件
│   │   └── base/                # 基础组件库
│   ├── views/                   # 页面
│   ├── stores/                  # Pinia 状态管理
│   ├── router/                  # Vue Router
│   ├── styles/                  # SCSS 样式
│   └── utils/                   # 工具函数
├── package.json                 # 前端依赖配置
├── vite.config.ts               # Vite 配置
└── tsconfig.json                # TypeScript 配置
```

## Notes

### 技术栈

- **后端**: Rust + Tauri 2.x + axum (HTTP 服务器)
- **前端**: Vue 3 + TypeScript + Pinia + Vue Router + SCSS
- **打包**: Tauri 内置打包为 Windows MSI/NSIS 安装包

### 核心功能

1. **反向代理服务**: 监听本地端口，转发请求到目标地址
2. **自动重试**: 失败时按配置自动重试（次数、延迟、状态码可配置）
3. **流式请求支持**: 支持 SSE 流式响应转发
4. **图形化配置**: 无需手动编辑配置文件
5. **多配置方案**: 支持保存和切换多套配置
6. **实时日志**: 查看请求/响应日志
7. **统计面板**: 请求成功率、重试次数等统计
8. **系统托盘**: 最小化到托盘，后台运行
9. **开机自启动**: 可选的开机自启动功能

### 开发命令

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建
npm run tauri build

# 前端 lint
npm run lint

# Rust 格式化
cargo fmt

# Rust 检查
cargo clippy
```
