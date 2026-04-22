export const logInfo = (message: string): void => {
  console.log(message)
}

export const logWarn = (message: string): void => {
  console.log(`⚠️ ${message}`)
}

export const logError = (message: string): void => {
  console.log(`❌ ${message}`)
}
