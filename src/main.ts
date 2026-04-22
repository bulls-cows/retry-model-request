import express, { type Request, type Response } from 'express'
import axios, { type AxiosRequestConfig, type AxiosResponse } from 'axios'
import { CONFIG, logInfo, logWarn, logError } from '#scripts/ConstantUtils.ts'

const app = express()
app.use(express.json())

const wait = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

const shouldRetry = (
  status: number | undefined,
  errorCode: string | undefined,
  retryConfig: RetryConfig
): boolean => {
  if (status && retryConfig.retryStatusCodes.includes(status)) return true
  if (errorCode === 'ECONNREFUSED' || errorCode === 'ETIMEDOUT') return true
  return false
}

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
        validateStatus: () => true,
      }

      logInfo(`📤 请求参数 | 第${attempt + 1}次 | ${targetUrl}`)
      logInfo(`   Method: ${requestConfig.method?.toUpperCase()}`)
      logInfo(`   Body: ${JSON.stringify(requestConfig.data, null, 2)}`)

      const response: AxiosResponse = await axios(requestConfig)

      logInfo(`📥 响应数据 | 第${attempt + 1}次 | ${targetUrl}`)
      logInfo(`   Status: ${response.status}`)
      logInfo(`   Data: ${JSON.stringify(response.data, null, 2)}`)
      logInfo(`✅ 请求成功 | 第${attempt + 1}次 | ${targetUrl}`)
      return res.status(response.status).send(response.data)
    } catch (err: unknown) {
      attempt++
      const axiosError = err as { response?: { status?: number; data?: unknown }; code?: string }
      const status = axiosError.response?.status
      const errorCode = axiosError.code

      if (!shouldRetry(status, errorCode, CONFIG.retry) || attempt > CONFIG.retry.maxRetries) {
        logError(`失败终止 | 状态: ${status || errorCode} | ${targetUrl}`)
        return res.status(status || 500).send(axiosError.response?.data || '服务请求失败')
      }

      logWarn(`重试中 | 第${attempt}/${CONFIG.retry.maxRetries}次 | ${targetUrl}`)
      await wait(CONFIG.retry.delayMs)
    }
  }
})

app.listen(CONFIG.localPort, () => {
  logInfo(`🚀 TypeScript 本地代理服务已启动`)
  logInfo(`👉 本地地址：http://localhost:${CONFIG.localPort}`)
  logInfo(`🎯 目标接口：${CONFIG.targetBaseUrl}`)
  logInfo(`🔁 最大重试：${CONFIG.retry.maxRetries} 次`)
})
