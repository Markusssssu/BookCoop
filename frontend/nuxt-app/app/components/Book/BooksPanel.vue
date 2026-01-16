<template>
  <UContainer class="py-8">
    <!-- Заголовок + кнопка добавить книгу -->
    <BooksHeader @created="refreshBooks" />

    <!-- Сетка книг -->
    <BooksGrid :books="books" :pending="pending" @delete="deleteBook" />

    <!-- Модальное окно с формой создания книги -->
    <BookCreateModal v-model="isModalOpen" @created="refreshBooks" />
  </UContainer>
</template>

<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { useBooks } from '~/composables/useBooks'
import BooksHeader from './BooksHeader.vue'
import BooksGrid from './BooksGrid.vue'
import BookCreateModal from './BookCreateModal.vue'

const { books, fetchBooks, loading } = useBooks() // <- реактивный массив books
const isModalOpen = ref(false)
const pending = ref(true)

// pending синхронизирован с состоянием загрузки
watchEffect(() => {
  pending.value = loading.value
})

// начальная загрузка
fetchBooks()

async function refreshBooks() {
  await fetchBooks() // обновляем массив реактивно
}

async function deleteBook(id: number) {
  if (!confirm('Вы уверены, что хотите удалить эту книгу?')) return
  try {
    await fetch(`/api/books/${id}`, { method: 'DELETE' })
    await refreshBooks() // сразу обновляем список после удаления
  } catch (e) {
    console.error('Ошибка при удалении:', e)
  }
}
</script>
