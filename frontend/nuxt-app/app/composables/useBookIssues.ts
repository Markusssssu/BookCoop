// composables/useBookIssues.ts
import { ref } from 'vue'
import type { BookIssue, NewBookIssue } from '~/types'

export function useBookIssues() {
    const issues = ref<BookIssue[]>([])
    const loading = ref(false)
    const error = ref<string | null>(null)

    const fetchIssues = async () => {
        loading.value = true
        error.value = null
        try {
            const res = await fetch('/api/book_issues')
            if (!res.ok) throw new Error('Failed to fetch book issues')
            issues.value = await res.json()
        } catch (e: any) {
            error.value = e.message
        } finally {
            loading.value = false
        }
    }

    const createIssue = async (payload: NewBookIssue) => {
        try {
            const res = await fetch('/api/book_issues', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            })
            if (!res.ok) throw new Error('Failed to create book issue')
            const newIssue = await res.json()
            issues.value.push(newIssue)
        } catch (e: any) {
            error.value = e.message
        }
    }

    return { issues, loading, error, fetchIssues, createIssue }
}
