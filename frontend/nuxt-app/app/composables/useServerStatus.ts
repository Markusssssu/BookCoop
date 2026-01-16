export const useServerStatus = () => {
  const online = useState<boolean>('server-online', () => true)
  const checking = useState<boolean>('server-checking', () => false)

  const config = useRuntimeConfig()
  // Берем URL из конфига, который пробрасывается через ENV
  const API_URL = "http://localhost:8080/api/health"

  const check = async () => {
    if (checking.value) return
    checking.value = true

    try {
      // Используем $fetch.raw, чтобы получить доступ к статусу ответа
      await $fetch.raw(API_URL, {
        method: 'GET', // GET надежнее для CORS, если бэкенд не ваш
        timeout: 3000,
        // Исключаем влияние кэша браузера
        params: { _t: Date.now() }
      })
      online.value = true
    } catch (err) {
      online.value = false
    } finally {
      checking.value = false
    }
  }

  // Правильный жизненный цикл для Nuxt 3/4
  if (import.meta.client) {
    onMounted(() => {
      check()
      const interval = setInterval(check, 30_000) // 10 сек слишком часто для 2026
      onUnmounted(() => clearInterval(interval))
    })
  }

  return { online, checking, check }
}
