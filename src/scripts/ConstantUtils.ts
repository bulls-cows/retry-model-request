export const LOCAL_PORT: number = parseInt(process.env.LOCAL_PORT!, 10)

export const TARGET_BASE_URL: string = process.env.TARGET_BASE_URL!

export const MAX_RETRIES: number = parseInt(process.env.MAX_RETRIES!, 10)

export const DELAY_MS: number = parseInt(process.env.DELAY_MS!, 10)

export const RETRY_STATUS_CODES: number[] = process.env
  .RETRY_STATUS_CODES!.split(',')
  .map(code => parseInt(code.trim(), 10))

export const CONFIG: Config = {
  localPort: LOCAL_PORT,
  targetBaseUrl: TARGET_BASE_URL,
  retry: {
    maxRetries: MAX_RETRIES,
    delayMs: DELAY_MS,
    retryStatusCodes: RETRY_STATUS_CODES,
  },
}
