import express, { type Request, type Response } from 'express'
import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios'

const CONFIG = {
  localPort: parseInt(process.env.LOCAL_PORT || '1050', 10),
  targetBaseUrl: process.env.TARGET_BASE_URL || 'https://你的-coding-plan-接口地址.com',
  retry: {
    maxRetries: parseInt(process.env.MAX_RETRIES || '5', 10),
    delayMs: parseInt(process.env.DELAY_MS || '2000', 10),
    retryStatusCodes: (process.env.RETRY_STATUS_CODES || '429,500,502,503,504')
      .split(',')
      .map(code => parseInt(code.trim(), 10)),
  },
}

type RetryConfig = typeof CONFIG.retry

const app = express()
app.use(express.json())

// 重试等待工具函数
const wait = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

// 判断是否需要重试
const shouldRetry = (
  status: number | undefined,
  errorCode: string | undefined,
  retryConfig: RetryConfig
): boolean => {
  if (status && retryConfig.retryStatusCodes.includes(status)) return true
  if (errorCode === 'ECONNREFUSED' || errorCode === 'ETIMEDOUT') return true
  return false
}

// 核心代理逻辑
app.all('{*path}', async (req: Request, res: Response) => {
  const path = req.originalUrl
  const targetUrl = `${CONFIG.targetBaseUrl}${path}`
  let attempt = 0

  while (attempt <= CONFIG.retry.maxRetries) {
    try {
      const requestConfig: AxiosRequestConfig = {
        method: req.method as AxiosRequestConfig['method'],
        url: targetUrl,
        data: req.body,
        headers: {
          ...req.headers,
          host: new URL(CONFIG.targetBaseUrl).host,
        },
        timeout: 60000,
        validateStatus: () => true, // 不自动抛错，手动判断状态码
      }

      const response: AxiosResponse = await axios(requestConfig)

      // 请求成功
      console.log(`✅ 请求成功 | 第${attempt + 1}次 | ${targetUrl}`)
      return res.status(response.status).send(response.data)
    } catch (err: unknown) {
      attempt++
      const axiosError = err as { response?: { status?: number; data?: unknown }; code?: string }
      const status = axiosError.response?.status
      const errorCode = axiosError.code

      // 不满足重试条件
      if (!shouldRetry(status, errorCode, CONFIG.retry) || attempt > CONFIG.retry.maxRetries) {
        console.log(`❌ 失败终止 | 状态: ${status || errorCode} | ${targetUrl}`)
        return res.status(status || 500).send(axiosError.response?.data || '服务请求失败')
      }

      // 执行重试
      console.log(`⚠️ 重试中 | 第${attempt}/${CONFIG.retry.maxRetries}次 | ${targetUrl}`)
      await wait(CONFIG.retry.delayMs)
    }
  }
})

// 启动服务
app.listen(CONFIG.localPort, () => {
  console.log(`🚀 TypeScript 本地代理服务已启动`)
  console.log(`👉 本地地址：http://localhost:${CONFIG.localPort}`)
  console.log(`🎯 目标接口：${CONFIG.targetBaseUrl}`)
  console.log(`🔁 最大重试：${CONFIG.retry.maxRetries} 次`)
})
