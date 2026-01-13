export const useServerStatus = () => {
  const online = useState<boolean>('server-online', () => false)
  const checking = useState<boolean>('server-checking', () => false)

  const check = async () => {
    if (checking.value) return

    checking.value = true

    try {
      await $fetch('/api/health', {
        timeout: 2000
      })

      online.value = true
    } catch (e) {
      online.value = false
    } finally {
      checking.value = false
    }
  }

  if (process.client) {
    setInterval(check, 10_000)
  }

  check()

  return {
    online,
    checking,
    check
  }
}
