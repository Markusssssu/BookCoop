<template>
  <div class="flex justify-between items-center mb-6">
    <h1 class="text-3xl font-bold">Каталог книг</h1>

    <UModal title="Новая книга" v-model="model">
      <UButton label="Добавить книгу" icon="i-lucide-plus" color="primary" />

      <template #content>
        <div class="p-4">
          <BookForm @success="$emit('created')" />
        </div>
      </template>
    </UModal>
  </div>
</template>

<script setup lang="ts">
import BookForm from './BookForm.vue'

const model = defineModel<boolean>()
defineEmits(['created'])
</script>
