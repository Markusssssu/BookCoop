<template>
  <div class="py-4 flex items-center justify-between">
    <div class="flex items-center gap-3">
      <UAvatar
          :alt="author.name"
          size="lg"
          background="bg-primary-500"
      />

      <div>
        <div class="font-medium text-gray-900 dark:text-white">
          {{ author.name }}
        </div>
        <div class="text-sm text-gray-500">
          {{ author.birth_date }} • {{ author.description }}
        </div>
        <div class="text-xs text-primary-500 mt-1">
          Книг в базе: {{ author.books_count }}
        </div>
      </div>
    </div>

    <UButton label="Профиль" variant="outline" size="sm" color="neutral" />
  </div>
</template>

<script setup lang="ts">
defineProps<{ author: any }>()
</script>
