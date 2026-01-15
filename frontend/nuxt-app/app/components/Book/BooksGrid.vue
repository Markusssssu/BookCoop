<template>
  <div v-if="pending" class="flex justify-center py-20">
    <UIcon name="i-lucide-loader-2" class="animate-spin w-10 h-10 text-primary" />
  </div>

  <div
      v-else-if="books?.length"
      class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6"
  >
    <BookCard
        v-for="book in books"
        :key="book.id"
        :book="book"
        @delete="$emit('delete', book.id)"
    />
  </div>

  <div
      v-else
      class="text-center py-20 bg-gray-50 dark:bg-gray-900 rounded-xl border-2 border-dashed border-gray-200 dark:border-gray-800"
  >
    <UIcon name="i-lucide-library" class="w-12 h-12 mx-auto mb-4 opacity-20" />
    <p class="text-gray-500">В каталоге пока нет ни одной книги</p>
  </div>
</template>

<script setup lang="ts">
import BookCard from './BookCard.vue'

defineProps<{ books: any[]; pending: boolean }>()
defineEmits(['delete'])
</script>
