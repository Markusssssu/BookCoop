<template>
  <UCard class="w-full max-w-lg mx-auto">
    <template #header>
      <h3 class="text-xl font-bold">Добавить новую книгу</h3>
    </template>

    <UForm :state="state" @submit="onSubmit" class="space-y-4">
      <UFormField label="Название книги" name="title" required>
        <UInput v-model="state.title" placeholder="Мастер и Маргарита" class="w-full" />
      </UFormField>

      <UFormField label="Автор" name="author">
        <UInput v-model="state.author" placeholder="Михаил Булгаков" />
      </UFormField>

      <div class="grid grid-cols-2 gap-4">
        <UFormField label="Жанр" name="genre">
          <USelect v-model="state.genre" :items="genres" />
        </UFormField>

        <UFormField label="Кол-во страниц" name="pages">
          <UInput v-model="state.pages" type="number" />
        </UFormField>
      </div>

      <UButton type="submit" block color="primary" icon="i-lucide-plus">
        Сохранить книгу
      </UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
const state = reactive({
  title: '',
  author: '',
  genre: 'fiction',
  pages: 100
})

const genres = [
  { label: 'Фантастика', value: 'fiction' },
  { label: 'Наука', value: 'science' },
  { label: 'Биография', value: 'biography' }
]

const onSubmit = (event: any) => {
  console.log('Данные книги:', event.data)
}
</script>