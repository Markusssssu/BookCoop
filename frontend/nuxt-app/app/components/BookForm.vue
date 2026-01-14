<template>
  <div class="flex flex-col items-center p-4 gap-8">
    <!-- КАРТОЧКА ФОРМЫ -->
    <UCard class="w-full max-w-xl bg-white/80 dark:bg-gray-900/80 backdrop-blur border border-gray-200 dark:border-gray-800 shadow-xl">
      <template #header>
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center">
            <UIcon name="i-lucide-book-plus" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h2 class="text-lg font-semibold">Добавить книгу</h2>
            <p class="text-sm text-gray-500">Заполните данные для добавления в библиотеку</p>
          </div>
        </div>
      </template>

      <UForm :state="state" :schema="schema" @submit="onSubmit" class="space-y-6">
        <UFormField label="Название книги" name="title">
          <UInput v-model="state.title" size="lg" placeholder="Мастер и Маргарита" icon="i-lucide-type" />
        </UFormField>

        <UFormField label="Автор" name="author">
          <UInput v-model="state.author" size="lg" placeholder="Михаил Булгаков" icon="i-lucide-user" />
        </UFormField>

        <div class="grid grid-cols-2 gap-4">
          <UFormField label="Жанр" name="genre">
            <USelectMenu v-model="state.genre" :options="genres" value-attribute="value" option-attribute="label" size="lg" />
          </UFormField>

          <UFormField label="Страниц" name="pages">
            <UInput v-model.number="state.pages" type="number" min="1" size="lg" icon="i-lucide-hash" />
          </UFormField>
        </div>

        <UButton type="submit" block size="lg" color="primary" icon="i-lucide-save" :loading="loading">
          Сохранить книгу
        </UButton>
      </UForm>
    </UCard>

    <!-- СПИСОК КНИГ -->
    <div class="w-full max-w-4xl">
      <div class="flex items-center gap-2 mb-4">
        <UIcon name="i-lucide-library" class="w-5 h-5 text-gray-500" />
        <h3 class="text-xl font-bold">Ваша библиотека</h3>
      </div>

      <div v-if="books.length > 0" class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UCard v-for="book in books" :key="book.id" class="transition-all hover:ring-2 hover:ring-primary/50">
          <div class="flex flex-col gap-2">
            <div class="flex justify-between items-start">
              <span class="font-bold text-lg leading-tight">{{ book.title }}</span>
              <UBadge size="xs" variant="subtle" color="primary">{{ book.genre }}</UBadge>
            </div>
            <p class="text-sm text-gray-500 italic">Автор: {{ book.author }}</p>
            <div class="flex items-center gap-1 text-xs text-gray-400 mt-2">
              <UIcon name="i-lucide-layers" />
              {{ book.page_count }} стр.
            </div>
          </div>
        </UCard>
      </div>

      <!-- Состояние пустой библиотеки -->
      <div v-else-if="!loadingList" class="flex flex-col items-center justify-center py-12 border-2 border-dashed border-gray-200 dark:border-gray-800 rounded-2xl">
        <UIcon name="i-lucide-ghost" class="w-10 h-10 text-gray-300 mb-2" />
        <p class="text-gray-400">Библиотека пуста. Добавьте первую книгу!</p>
      </div>
      
      <div v-else class="flex justify-center py-12">
        <UIcon name="i-lucide-loader-2" class="animate-spin w-8 h-8 text-primary" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue'
import { z } from 'zod'

// --- СОСТОЯНИЕ ---
const API_URL = 'http://localhost:8080/api/books'
const loading = ref(false)
const loadingList = ref(false)
const books = ref<any[]>([])

const genres = [
  { label: 'Фантастика', value: 'Фантастика' },
  { label: 'Наука', value: 'Наука' },
  { label: 'Биография', value: 'Биография' },
  { label: 'Классика', value: 'Классика' }
]

// --- ВАЛИДАЦИЯ ---
const schema = z.object({
  title: z.string().min(2, 'Введите название'),
  author: z.string().min(2, 'Введите автора'),
  genre: z.string(),
  pages: z.number().min(1, 'Минимум 1 страница')
})

const initialState = {
  title: '',
  author: '',
  genre: 'Фантастика',
  pages: 100
}

const state = reactive({ ...initialState })

async function fetchBooks() {
  loadingList.value = true
  try {
    const data = await $fetch<any[]>(API_URL)
    books.value = data
  } catch (e) {
    console.error('Ошибка загрузки:', e)
  } finally {
    loadingList.value = false
  }
}

async function onSubmit() {
  loading.value = true
  try {
    const payload = {
      title: state.title,
      author: state.author,
      genre: state.genre,
      page_count: state.pages 
    }

    await $fetch(API_URL, {
      method: 'POST',
      body: payload
    })

    await fetchBooks()
    Object.assign(state, initialState)
    
    
  } catch (e: any) {
    alert('Ошибка при сохранении: ' + (e.data?.message || e.message))
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchBooks()
})
</script>
