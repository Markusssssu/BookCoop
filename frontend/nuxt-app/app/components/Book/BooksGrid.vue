<template>
  <div v-if="loading" class="flex justify-center py-20">
    <UIcon name="i-lucide-loader-2" class="animate-spin w-10 h-10 text-primary" />
  </div>

  <div v-else-if="books.length" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
    <BookCard
        v-for="book in books"
        :key="book.book_id"
        :book="book"
        @delete="deleteBook(book.book_id)"
    />
  </div>

  <div v-else class="text-center py-20 bg-gray-50 dark:bg-gray-900 rounded-xl border-2 border-dashed border-gray-200 dark:border-gray-800">
    <UIcon name="i-lucide-library" class="w-12 h-12 mx-auto mb-4 opacity-20" />
    <p class="text-gray-500">В каталоге пока нет ни одной книги</p>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import BookCard from './BookCard.vue'
import { useBooks } from '~/composables/useBooks'

const { books, loading, error, fetchBooks } = useBooks()

// Загружаем книги при монтировании
onMounted(() => fetchBooks())

// Обработка удаления книги
async function deleteBook(id: number) {
  try {
    await fetch(`http://localhost:8080/api/books/${id}`, { method: 'DELETE' })
    fetchBooks() // обновляем список после удаления
  } catch (e) {
    console.error('Ошибка при удалении:', e)
  }
}
</script>
