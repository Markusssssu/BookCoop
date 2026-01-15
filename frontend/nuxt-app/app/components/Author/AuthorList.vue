<template>
  <UCard>
    <UScrollArea class="h-[500px] px-4">
      <div v-if="pending" class="flex justify-center py-10">
        <UIcon name="i-lucide-loader-2" class="animate-spin w-6 h-6 text-primary" />
      </div>

      <div v-else class="divide-y divide-gray-200 dark:divide-gray-800">
        <AuthorListItem
            v-for="author in authors"
            :key="author.id"
            :author="author"
        />
      </div>
    </UScrollArea>
  </UCard>
</template>

<script setup lang="ts">
import AuthorListItem from './AuthorListItem.vue'

defineProps<{
  authors: any[]
  pending: boolean
}>()
</script>
