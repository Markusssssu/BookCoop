import { ref } from 'vue'

const API_URL = 'http://localhost:8080/api/books'

export function useBooks() {
  const books = ref<any[]>([])
  const loadingList = ref(false)

  async function fetchBooks() {
    loadingList.value = true
    try {
      books.value = await $fetch(API_URL)
    } finally {
      loadingList.value = false
    }
  }

  async function createBook(payload: any) {
    await $fetch(API_URL, {
      method: 'POST',
      body: payload
    })
    await fetchBooks()
  }

  async function deleteBook(payload: any) {
    await $fetch(API_URL, {
      method: "DELETE",
      body: payload
    })
  }

  return {
    books,
    loadingList,
    fetchBooks,
    createBook
  }
}
