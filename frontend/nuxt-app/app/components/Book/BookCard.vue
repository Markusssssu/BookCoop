<template>
  <UCard class="hover:ring-2 ring-primary-500 transition-all cursor-default">
    <div class="flex gap-4">
      <div class="w-24 h-32 bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center flex-shrink-0">
        <UIcon name="i-lucide-book-open" class="text-3xl opacity-20" />
      </div>

      <div class="flex-1 min-w-0">
        <h3 class="font-bold text-lg truncate" :title="book.title">{{ book.title }}</h3>
        <p class="text-sm text-gray-500 truncate">{{ book.author }}</p>

        <div class="flex flex-wrap gap-2 mt-2">
          <UBadge :label="book.genre" variant="subtle" size="sm" color="primary" />
          <span class="text-xs text-gray-400 flex items-center">
            <UIcon name="i-lucide-hash" class="mr-1" />
            {{ book.page_count }} стр.
          </span>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton icon="i-lucide-pencil" variant="ghost" color="neutral" size="sm" />
        <UButton
            icon="i-lucide-trash"
            variant="ghost"
            color="error"
            size="sm"
            @click="$emit('delete')"
        />
      </div>
    </template>
  </UCard>
</template>

<script setup lang="ts">
defineProps<{ book: any }>()
defineEmits(['delete'])
</script>
