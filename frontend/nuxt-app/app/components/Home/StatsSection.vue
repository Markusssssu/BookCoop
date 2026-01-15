<template>
  <section class="py-20">
    <UContainer>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <UCard>
          <p class="text-sm text-gray-500">Активные пользователи</p>
          <p class="text-3xl font-bold">{{ usersActive }}</p>
        </UCard>

        <UCard>
          <p class="text-sm text-gray-500">Запросы / сек</p>
          <p class="text-3xl font-bold">{{ requestsPerSec }}</p>
        </UCard>

        <UCard>
          <p class="text-sm text-gray-500">Стабильность сервера</p>
          <p class="text-3xl font-bold text-primary-500">{{ serverUptime }}</p>
        </UCard>
      </div>
    </UContainer>
  </section>
</template>

<script setup lang="ts">
const usersActive = 0
const requestsPerSec = 0
const serverUptime = '0.0%'
</script>
