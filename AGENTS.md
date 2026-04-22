# AGENTS.md

## Scope

- 本仓库默认语言: TypeScript
- 允许修改目录: `src/` 目录下的所有 `.ts` 文件、配置文件
- 禁止修改目录: 无

## Quality Gate

- 改动后必须执行:
  - `npm run lint` (代码格式化、静态检查、类型检查)

## Delivery Format

- 文件引用格式: `[文件名](file:///绝对路径#L行号)`
- 修改代码前先说明修改意图

## Project Structure

```
retry-model-request/
├── src/
│   └── main.ts           # 主程序 - 反向代理服务器，支持自动重试
├── package.json         # 项目配置
├── tsconfig.json        # TypeScript 配置
├── .nvmrc               # Node.js 版本锁定 (v22.18.0)
└── .npmrc               # npm 配置 (淘宝镜像、精确版本)
```

## Notes

### Node.js 版本要求

- 项目使用 Node.js v22.18.0
- **重要**: Node.js 22.x 原生支持执行 TypeScript 文件（通过 `--experimental-strip-types` 或默认支持），无需安装 ts-node 等第三方运行时
- 启动命令: `npm start` 或 `node src/main.ts`

### 项目功能

- 对 coding plan、token plan 的反向代理
- 在供应商接口报错时自动重试，避免客户端中断执行任务
- 支持配置重试次数、延迟、重试状态码
