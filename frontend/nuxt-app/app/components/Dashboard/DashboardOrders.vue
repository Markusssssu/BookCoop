<template>
  <UCard title="Последние операции">
    <div class="space-y-4">
      <div
          v-for="order in orders"
          :key="order.id"
          class="flex items-center justify-between border-b border-gray-100 dark:border-gray-800 pb-3 last:border-0"
      >
        <div class="flex flex-col">
          <span class="text-sm font-bold">{{ order.book }}</span>
          <span class="text-xs text-gray-500">
            ID: {{ order.id }} • {{ order.user }}
          </span>
        </div>

        <UBadge
            :color="order.status === 'Выдано' ? 'primary' : 'neutral'"
            variant="subtle"
            size="sm"
        >
          {{ order.status }}
        </UBadge>
      </div>

      <UButton block variant="ghost" size="sm">
        Все транзакции
      </UButton>
    </div>
  </UCard>
</template>

<script setup lang="ts">
defineProps<{ orders: any[] }>()
</script>
