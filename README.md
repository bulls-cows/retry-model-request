# retry-model-request

对 coding plan、token plan 的反向代理，在供应商接口报错时自动重试，避免客户端由于报错而中断执行任务。

## 特性

- **自动重试**：当目标接口返回指定状态码或网络错误时自动重试
- **可配置**：支持自定义重试次数、延迟时间、触发重试的状态码
- **透明代理**：完全转发请求和响应，对客户端透明
- **实时日志**：输出请求状态日志，便于调试和监控

## 环境要求

- Node.js >= 22.18.0

## 安装

```bash
npm install
```

## 配置

在项目根目录创建 `.env` 文件：

```env
LOCAL_PORT=3000
TARGET_BASE_URL=https://api.example.com
MAX_RETRIES=3
DELAY_MS=1000
RETRY_STATUS_CODES=429,500,502,503,504
```

### 配置项说明

| 配置项               | 说明                               | 示例                      |
| -------------------- | ---------------------------------- | ------------------------- |
| `LOCAL_PORT`         | 本地代理服务端口                   | `3000`                    |
| `TARGET_BASE_URL`    | 目标接口地址                       | `https://api.example.com` |
| `MAX_RETRIES`        | 最大重试次数                       | `3`                       |
| `DELAY_MS`           | 重试延迟（毫秒）                   | `1000`                    |
| `RETRY_STATUS_CODES` | 触发重试的 HTTP 状态码（逗号分隔） | `429,500,502,503,504`     |

## 使用

### 启动服务

```bash
npm start
```

### 开发模式（热重载）

```bash
npm run dev
```

启动成功后会显示：

```
🚀 TypeScript 本地代理服务已启动
👉 本地地址：http://localhost:3000
🎯 目标接口：https://api.example.com
🔁 最大重试：3 次
```

### 调用示例

将原本请求目标接口的地址改为本地代理地址：

```bash
# 原请求
curl https://api.example.com/v1/chat/completions

# 通过代理请求
curl http://localhost:3000/v1/chat/completions
```

## 技术栈

- [Express](https://expressjs.com/) - Web 框架
- [Axios](https://axios-http.com/) - HTTP 客户端
- [TypeScript](https://www.typescriptlang.org/) - 类型安全

## License

本项目采用 [Apache 2.0](LICENSE) 协议开源。
