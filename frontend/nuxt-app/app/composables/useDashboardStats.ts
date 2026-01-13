export const useDashboardStats = () => {
  const config = useRuntimeConfig()

  const data = useState<any>('dashboard', () => null)
  const loading = useState<boolean>('dashboard-loading', () => false)

  const fetchStats = async () => {
    loading.value = true

    try {
      data.value = await $fetch(`${config.public.apiBase}/api/dashboard`)
    } catch {
      data.value = null
    } finally {
      loading.value = false
    }
  }

  if (process.client && !data.value) {
    fetchStats()
  }

  return {
    data,
    loading,
    fetchStats
  }
}
