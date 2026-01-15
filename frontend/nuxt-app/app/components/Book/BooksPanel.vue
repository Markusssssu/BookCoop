<template>
  <UContainer class="py-8">
    <!-- Заголовок + кнопка добавить книгу -->
    <BookBooksHeader v-model="isModalOpen" @created="refreshBooks" />

    <!-- Сетка книг -->
    <BookBooksGrid :books="books" :pending="pending" @delete="deleteBook" />

    <!-- Модальное окно с формой создания книги -->
    <BookCreateModal v-model="isModalOpen" @created="refreshBooks" />
  </UContainer>
</template>

<script setup lang="ts">
const { data: books, refresh, pending } =
    await useFetch('http://localhost:8080/api/books')

const isModalOpen = ref(false)

async function deleteBook(id: number) {
  if (!confirm('Вы уверены, что хотите удалить эту книгу?')) return

  try {
    await $fetch(`http://localhost:8080/api/books/${id}`, { method: 'DELETE' })
    refresh()
  } catch (e) {
    console.error('Ошибка при удалении:', e)
  }
}

function refreshBooks() {
  refresh()
}
</script>
