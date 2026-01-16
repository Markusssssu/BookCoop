<template>
  <UContainer class="py-8">
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Левая колонка -->
      <div class="lg:col-span-1">
        <h2 class="text-2xl font-bold mb-4">Регистрация</h2>
        <AuthorForm @success="refreshAuthors" />
      </div>

      <!-- Правая колонка -->
      <div class="lg:col-span-2">
        <h2 class="text-2xl font-bold mb-4">Все авторы</h2>
        <AuthorList :authors="authorsList" :pending="loading" />
      </div>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import AuthorForm from './AuthorForm.vue'
import AuthorList from './AuthorList.vue'
import { useAuthors } from '~/composables/useAuthors'

const { authors, fetchAuthors } = useAuthors()
const authorsList = authors
const loading = ref(true)

const refreshAuthors = async () => {
  loading.value = true
  await fetchAuthors()
  loading.value = false
}

onMounted(async () => {
  await refreshAuthors()
})
</script>
