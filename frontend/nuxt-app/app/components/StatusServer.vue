<script setup lang="ts">
const { online, checking } = useServerStatus()
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <UCard class="bg-gray-900 text-white">
      <h4 class="font-bold mb-2">Статус сервера</h4>

      <div class="flex items-center gap-2">
        <div
          class="w-2 h-2 rounded-full"
          :class="[
            online ? 'bg-green-500 animate-pulse' : 'bg-red-500',
            checking && 'opacity-50'
          ]"
        />

        <span class="text-sm opacity-80">
          <template v-if="checking">
            Проверка соединения…
          </template>
          <template v-else-if="online">
            Система стабильна
          </template>
          <template v-else>
            Сервер недоступен
          </template>
        </span>
      </div>
    </UCard>
  </div>
</template>
