export const useServerStatus = () => {
  const online = useState<boolean>('server-online', () => false)
  const checking = useState<boolean>('server-checking', () => false)

  // Используем конфиг для получения URL бэкенда
  const config = useRuntimeConfig()
  // В 2026 лучше обращаться к публичному прокси или прямому IP/домену
  const API_URL = 'http://localhost:8080/api/health'

  const check = async () => {
    if (checking.value) return
    checking.value = true

    try {
      await $fetch(API_URL, {
        method: 'HEAD',
        timeout: 2000,
      })
      online.value = true
    } catch (err) {
      online.value = false
    } finally {
      checking.value = false
    }
  }

  if (process.client) {
    onMounted(() => {
      check() // Первая проверка
      const interval = setInterval(check, 10_000)

      onUnmounted(() => clearInterval(interval))
    })
  }

  return {
    online,
    checking,
    check
  }
}
