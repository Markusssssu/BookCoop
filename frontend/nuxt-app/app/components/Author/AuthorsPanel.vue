<template>
  <UContainer class="py-8">
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Левая колонка -->
      <div class="lg:col-span-1">
        <h2 class="text-2xl font-bold mb-4">Регистрация</h2>
        <AuthorForm @success="refresh" />
      </div>

      <!-- Правая колонка -->
      <div class="lg:col-span-2">
        <h2 class="text-2xl font-bold mb-4">Все авторы</h2>
        <AuthorList :authors="authorsList" :pending="pending" />
      </div>
    </div>
  </UContainer>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import AuthorList from './AuthorList.vue'

const authorsList = ref([

])

const pending = ref(false)
const refresh = () => console.log('Данные обновлены (статично)')
</script>
