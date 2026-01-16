// composables/useBooks.ts
import { ref } from 'vue'
import type { Book, NewBook } from '~/types'

export function useBooks() {
  const books = ref<Book[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchBooks = async () => {
    loading.value = true
    error.value = null
    try {
      const res = await fetch('http://localhost:8080/api/books')
      if (!res.ok) throw new Error('Failed to fetch books')
      books.value = await res.json()
    } catch (e: any) {
      error.value = e.message
    } finally {
      loading.value = false
    }
  }

  const createBook = async (payload: NewBook) => {
    try {
      const res = await fetch('http://localhost:8080/api/books', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      })
      if (!res.ok) throw new Error('Failed to create book')
      const newBook = await res.json()
      books.value.push(newBook)
    } catch (e: any) {
      error.value = e.message
    }
  }

  return { books, loading, error, fetchBooks, createBook }
}
