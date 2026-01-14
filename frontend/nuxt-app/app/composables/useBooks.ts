export const useBooks = () => {
  const config = useRuntimeConfig()
  const loading = ref(false)
  const error = ref<string | null>(null)

  const createBook = async (bookData: any) => {
    loading.value = true
    error.value = null
    
    try {
      const response = await $fetch('/api/books', {
        baseURL: 'http://localhost:8080', 
        method: 'POST',
        body: bookData,
      })
      return response
    } catch (err: any) {
      error.value = err.data?.message || 'Ошибка при сохранении книги'
      throw err
    } finally {
      loading.value = false
    }
  }

  return {
    createBook,
    loading,
    error
  }
}
