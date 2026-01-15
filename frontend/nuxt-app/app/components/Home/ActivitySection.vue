<template>
  <section class="py-20 bg-gray-50 dark:bg-gray-900/40 border-y border-gray-200 dark:border-gray-800">
    <UContainer>
      <div class="grid lg:grid-cols-3 gap-8">

        <!-- Последние действия -->
        <UCard class="lg:col-span-2">
          <template #header>
            <h2 class="text-lg font-semibold">Последние действия</h2>
          </template>

          <div class="space-y-2">
            <div
                v-for="log in recentActions"
                :key="log.time"
                class="flex items-center justify-between text-sm p-3 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800 transition"
            >
              <div class="flex items-center gap-3">
                <UAvatar :alt="log.user" size="xs" />
                <span>
                  <b class="text-primary-500">{{ log.user }}</b>
                  {{ log.action }}
                  <b>{{ log.target }}</b>
                </span>
              </div>
              <span class="text-gray-400">{{ log.time }}</span>
            </div>
          </div>
        </UCard>

        <!-- Статистика -->
        <div class="grid grid-cols-2 gap-4">
          <UCard>
            <p class="text-sm text-gray-500">Книг в базе</p>
            <p class="text-3xl font-bold">{{ stats.booksCount }}</p>
          </UCard>

          <UCard>
            <p class="text-sm text-gray-500">Авторы в базе</p>
            <p class="text-3xl font-bold">{{ stats.authorsCount }}</p>
          </UCard>

          <UCard class="col-span-2">
            <p class="text-sm text-gray-500">Выдано сегодня</p>
            <p class="text-4xl font-bold text-primary-500">{{ stats.issuedToday }}</p>
          </UCard>
        </div>

      </div>
    </UContainer>
  </section>
</template>

<script setup lang="ts">
const recentActions = [
  { user: '', action: '', target: '', time: '' }
]

const stats = {
  booksCount: 0,
  authorsCount: 0,
  issuedToday: 0
}
</script>
