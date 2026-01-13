export const useServerStatus = () => {
  const online = useState<boolean>('server-online', () => false)
  const checking = useState<boolean>('server-checking', () => false)
  const timer = useState<NodeJS.Timeout | null>('server-timer', () => null)

  const check = async () => {
    if (checking.value) return

    checking.value = true

    try {
      await $fetch('http://localhost:8080/api/health', {
        timeout: 2000,
        retry: 0,
        ignoreResponseError: true
      })

      online.value = true
    } catch {
      online.value = false
    } finally {
      checking.value = false
    }
  }

  if (process.client && !timer.value) {
    timer.value = setInterval(check, 10_000)
  }

  if (process.client) {
    check()
  }

  return {
    online,
    checking,
    check
  }
}
