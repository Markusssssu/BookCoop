<template>
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
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { z } from 'zod'

const API_URL = 'http://localhost:8080/api/books'
const loading = ref(false)
const genres = [
  { label: 'Фантастика', value: 'Фантастика' },
  { label: 'Наука', value: 'Наука' },
  { label: 'Биография', value: 'Биография' },
  { label: 'Классика', value: 'Классика' }
]

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
const emit = defineEmits(['success'])

async function onSubmit() {
  loading.value = true
  try {
    await $fetch(API_URL, {
      method: 'POST',
      body: {
        title: state.title,
        author: state.author,
        genre: state.genre,
        page_count: state.pages
      }
    })
    Object.assign(state, initialState)
    emit('success')
  } catch (e: any) {
    alert('Ошибка при сохранении: ' + (e.data?.message || e.message))
  } finally {
    loading.value = false
  }
}
</script>
