import { getDirname, joinPath } from 'nsuite'
import { readFileSync } from 'fs'

const __dirname = getDirname(import.meta.url)
export const PATH_ROOT = joinPath(__dirname, '../../')

// 从 package.json 同步读取应用名称
const packageJsonPath = joinPath(PATH_ROOT, 'package.json')
const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'))
export const APP_NAME: string = packageJson.name

// 服务配置
export const LOCAL_PORT: number = parseInt(process.env.LOCAL_PORT!, 10)

export const TARGET_BASE_URL: string = process.env.TARGET_BASE_URL!

export const MAX_RETRIES: number = parseInt(process.env.MAX_RETRIES!, 10)

export const DELAY_MS: number = parseInt(process.env.DELAY_MS!, 10)

export const RETRY_STATUS_CODES: number[] = process.env
  .RETRY_STATUS_CODES!.split(',')
  .map(code => parseInt(code.trim(), 10))

// 日志配置
export const NODE_ENV: string = process.env.NODE_ENV || 'development'

export const DEBUG: string = process.env.DEBUG || '0'

export const PATH_LOGS = joinPath(PATH_ROOT, 'logs')

export const CONFIG: Config = {
  localPort: LOCAL_PORT,
  targetBaseUrl: TARGET_BASE_URL,
  retry: {
    maxRetries: MAX_RETRIES,
    delayMs: DELAY_MS,
    retryStatusCodes: RETRY_STATUS_CODES,
  },
}
