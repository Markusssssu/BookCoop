<template>
  <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
    <UCard
        v-for="s in stats"
        :key="s.label"
        class="relative overflow-hidden"
    >
      <div class="flex justify-between items-start">
        <div>
          <p class="text-sm text-gray-500 font-medium">{{ s.label }}</p>
          <p class="text-2xl font-bold mt-1">{{ s.value }}</p>
        </div>
        <UIcon :name="s.icon" class="text-2xl text-primary-500/50" />
      </div>

      <div class="mt-4 flex items-center gap-1 text-xs">
        <span :class="s.delta.includes('+') ? 'text-green-500' : 'text-gray-400'">
          {{ s.delta }}
        </span>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
defineProps<{ stats: any[] }>()
</script>
