import { createLogger, joinPath } from 'nsuite'
import { PATH_LOGS, NODE_ENV, DEBUG, LOCAL_PORT, SITE_TITLE } from '#scripts/ConstantUtils.ts'

export const logger = createLogger({
  level: DEBUG === '1' ? 'debug' : NODE_ENV === 'development' ? 'info' : 'warn',
  meta: {
    server: SITE_TITLE || 'Unknown Server',
    PORT: String(LOCAL_PORT),
    NODE_ENV: NODE_ENV || 'unknown',
  },
  maxLength: 2000,
  filename: joinPath(PATH_LOGS, './application-%DATE%.log'),
  zippedArchive: false,
  enableConsole: true,
  includeCallSite: true,
  inspector: 'objectInspect',
})
