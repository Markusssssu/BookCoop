import { ref } from 'vue'
import type { Author, NewAuthor } from '~/types'

const API_URL = 'http://localhost:8080/api/authors'

export function useAuthors() {
    const authors = ref<Author[]>([])
    const loading = ref(false)
    const error = ref<string | null>(null)

    /* =======================
       GET
    ======================= */
    const fetchAuthors = async () => {
        loading.value = true
        error.value = null
        try {
            const res = await fetch(API_URL)
            if (!res.ok) throw new Error('Failed to fetch authors')
            authors.value = await res.json()
        } catch (e: any) {
            error.value = e.message
        } finally {
            loading.value = false
        }
    }

    /* =======================
       POST
    ======================= */
    const createAuthor = async (payload: NewAuthor) => {
        error.value = null
        try {
            const res = await fetch(API_URL, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            })

            if (!res.ok) throw new Error('Failed to create author')

            const newAuthor: Author = await res.json()
            authors.value.unshift(newAuthor) // сразу видно в списке
            return newAuthor
        } catch (e: any) {
            error.value = e.message
            throw e
        }
    }

    /* =======================
       PUT
    ======================= */
    const updateAuthor = async (id: number, payload: Partial<NewAuthor>) => {
        error.value = null
        try {
            const res = await fetch(`${API_URL}/${id}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            })

            if (!res.ok) throw new Error('Failed to update author')

            const updatedAuthor: Author = await res.json()

            const index = authors.value.findIndex(a => a.id === id)
            if (index !== -1) {
                authors.value[index] = updatedAuthor
            }

            return updatedAuthor
        } catch (e: any) {
            error.value = e.message
            throw e
        }
    }

    /* =======================
       DELETE
    ======================= */
    const deleteAuthor = async (id: number) => {
        error.value = null
        try {
            const res = await fetch(`${API_URL}/${id}`, {
                method: 'DELETE'
            })

            if (!res.ok) throw new Error('Failed to delete author')

            authors.value = authors.value.filter(a => a.id !== id)
        } catch (e: any) {
            error.value = e.message
            throw e
        }
    }

    return {
        authors,
        loading,
        error,
        fetchAuthors,
        createAuthor,
        updateAuthor,
        deleteAuthor
    }
}
