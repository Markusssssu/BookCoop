<template>
  <UCard
      class="w-full max-w-xl bg-white/80 dark:bg-gray-900/80 backdrop-blur
           border border-gray-200 dark:border-gray-800 shadow-xl"
  >
    <template #header>
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center">
          <UIcon name="i-lucide-book-plus" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h2 class="text-lg font-semibold">Добавить книгу</h2>
          <p class="text-sm text-gray-500">
            Заполните данные для добавления в библиотеку
          </p>
        </div>
      </div>
    </template>

    <UForm :state="state" @submit.prevent="onSubmit" class="space-y-6">
      <!-- Название -->
      <UFormField label="Название книги" name="title">
        <UInput
            v-model="state.title"
            size="lg"
            placeholder="Мастер и Маргарита"
            icon="i-lucide-type"
        />
      </UFormField>

      <!-- Автор -->
      <UFormField label="Автор" name="author">
        <UInput
            v-model="state.author"
            size="lg"
            placeholder="Михаил Булгаков"
            icon="i-lucide-user"
        />
      </UFormField>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-end">
        <UFormField label="Жанр" name="genre" class="w-full">
          <USelect
              v-model="state.genre"
              :options="genres"
              size="lg"
              class="w-full"
          />
        </UFormField>

        <UFormField label="Страниц" name="pages" class="w-full">
          <UInput
              v-model.number="state.pages"
              type="number"
              min="1"
              size="lg"
              icon="i-lucide-hash"
              class="w-full"
          />
        </UFormField>
      </div>

      <UButton
          type="submit"
          block
          size="lg"
          color="primary"
          icon="i-lucide-save"
          :loading="loading"
      >
        Сохранить книгу
      </UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useBooks } from '~/composables/useBooks'

const { createBook } = useBooks()
const emit = defineEmits(['success'])
const loading = ref(false)

/**
 * Никакого поиска.
 * Просто список.
 * Люди любят списки.
 */
const genres = [
  'Фантастика',
  'Классика',
  'Наука',
  'Биография',
  'История',
  'Фэнтези'
]

const initialState = {
  title: '',
  author: '',
  genre: 'Фантастика',
  pages: 100
}

const state = reactive({ ...initialState })

async function onSubmit() {
  loading.value = true
  try {
    await createBook({
      title: state.title,
      author: state.author,
      genre: state.genre,
      page_count: state.pages
    })

    Object.assign(state, initialState)
    emit('success') // сигнал наверх: «книга создана»
  } catch (e) {
    console.error('Ошибка при сохранении книги', e)
  } finally {
    loading.value = false
  }
}
</script>
