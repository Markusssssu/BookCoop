<template>
  <div class="py-4 flex items-start justify-between gap-4">
    <!-- Левая часть: аватар + инфа -->
    <div class="flex items-start gap-3 min-w-0">
      <UAvatar
          :alt="author.full_name"
          size="lg"
          background="bg-primary-500"
      />

      <div class="min-w-0">
        <div class="font-medium text-gray-900 dark:text-white truncate">
          {{ author.full_name }}
        </div>

        <div class="text-sm text-gray-500 truncate">
          {{ author.date_of_birth || 'Дата рождения неизвестна' }}
        </div>

        <div
            v-if="author.biography"
            class="text-sm text-gray-400 line-clamp-2"
        >
          {{ author.biography }}
        </div>

        <div class="text-xs text-primary-500 mt-1">
          Книг в базе: {{ author.books_count ?? 0 }}
        </div>
      </div>
    </div>

    <!-- Правая часть: действия -->
    <div class="flex items-center gap-2 shrink-0">
      <UButton
          size="xs"
          variant="ghost"
          color="primary"
          icon="i-lucide-pencil"
          @click="emit('edit', author)"
      >
        Изменить
      </UButton>

      <UButton
          size="xs"
          variant="ghost"
          color="red"
          icon="i-lucide-trash"
          @click="emit('delete', author.author_id)"
      >
        Удалить
      </UButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Author } from '~/types'

defineProps<{
  author: Author
}>()

const emit = defineEmits<{
  (e: 'edit', author: Author): void
  (e: 'delete', id: number): void
}>()
</script>
