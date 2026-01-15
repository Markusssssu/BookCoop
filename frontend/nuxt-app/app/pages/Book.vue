<template>
  <UContainer class="py-8">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Каталог книг</h1>

      <!-- Модальное окно для добавления -->
      <UModal title="Новая книга" v-model="isModalOpen">
        <UButton label="Добавить книгу" icon="i-lucide-plus" color="primary" />

        <template #content>
          <div class="p-4">
            <!-- Передаем функцию обновления в компонент формы, если нужно закрывать после успеха -->
            <BookForm @success="onBookAdded" />
          </div>
        </template>
      </UModal>
    </div>

    <!-- Индикатор загрузки -->
    <div v-if="pending" class="flex justify-center py-20">
      <UIcon name="i-lucide-loader-2" class="animate-spin w-10 h-10 text-primary" />
    </div>

    <!-- Сетка книг (отображается, когда данные загружены) -->
    <div v-else-if="books && books.length > 0" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <UCard
        v-for="book in books"
        :key="book.id"
        class="hover:ring-2 ring-primary-500 transition-all cursor-default"
      >
        <div class="flex gap-4">
          <!-- Заглушка обложки -->
          <div class="w-24 h-32 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center flex-shrink-0">
            <UIcon name="i-lucide-book-open" class="text-3xl opacity-20" />
          </div>

          <div class="flex-1 min-w-0">
            <h3 class="font-bold text-lg truncate" :title="book.title">
              {{ book.title }}
            </h3>
            <p class="text-sm text-gray-500 truncate">{{ book.author }}</p>
            <div class="flex flex-wrap gap-2 mt-2">
              <UBadge :label="book.genre" variant="subtle" size="sm" color="primary" />
              <span class="text-xs text-gray-400 flex items-center">
                <UIcon name="i-lucide-hash" class="mr-1" /> {{ book.page_count }} стр.
              </span>
            </div>
          </div>
        </div>

        <template #footer>
          <div class="flex justify-end gap-2">
            <UButton icon="i-lucide-pencil" variant="ghost" color="neutral" size="sm" />
            <UButton
              icon="i-lucide-trash"
              variant="ghost"
              color="error"
              size="sm"
              @click="deleteBook(book.id)"
            />
          </div>
        </template>
      </UCard>
    </div>

    <!-- Состояние, если книг нет -->
    <div v-else class="text-center py-20 bg-gray-50 dark:bg-gray-900 rounded-xl border-2 border-dashed border-gray-200 dark:border-gray-800">
      <UIcon name="i-lucide-library" class="w-12 h-12 mx-auto mb-4 opacity-20" />
      <p class="text-gray-500">В каталоге пока нет ни одной книги</p>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
const { data: books, refresh, pending } = await useFetch('http://localhost:8080/api/books')

const isModalOpen = ref(false)

function onBookAdded() {
  isModalOpen.value = false
  refresh()
}

async function deleteBook(id: number) {
  if (!confirm('Вы уверены, что хотите удалить эту книгу?')) return

  try {
    await $fetch(`http://localhost:8080/api/books/${id}`, { method: 'DELETE' })
    refresh()
  } catch (e) {
    console.error('Ошибка при удалении:', e)
  }
}
</script>
