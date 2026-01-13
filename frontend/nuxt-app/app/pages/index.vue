<template>
  <div class="space-y-24">

    <!-- HERO -->
    <section class="py-24 border-b border-gray-200 dark:border-gray-800">
      <UContainer>
        <div class="max-w-4xl mx-auto text-center space-y-6">
          <UBadge color="primary" variant="soft">
            NuxtJs + Axum 
          </UBadge>

          <h1 class="text-5xl font-bold tracking-tight">
            Библиотека в реальном времени
          </h1>

          <p class="text-lg text-gray-500">
            Управляйте книгами и авторами через реактивный интерфейс с мгновенной синхронизацией.
          </p>

          <div class="flex justify-center gap-3 pt-4">
            <UButton size="lg" icon="i-lucide-plus" to="/book">
              Начать
            </UButton>
            <UButton size="lg" variant="soft" icon="i-lucide-bar-chart-3" to="/dashboard">
              Аналитика
            </UButton>
          </div>
        </div>
      </UContainer>
    </section>

    <!-- FEATURES -->
    <section>
      <UContainer>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <UCard
            v-for="item in features"
            :key="item.title"
            class="hover:shadow-md transition"
          >
            <div class="space-y-4">
              <div class="w-10 h-10 flex items-center justify-center rounded-lg bg-primary-500/10">
                <UIcon :name="item.icon" class="text-primary-500 text-xl" />
              </div>

              <h3 class="text-lg font-semibold">
                {{ item.title }}
              </h3>

              <p class="text-sm text-gray-500">
                {{ item.description }}
              </p>
            </div>
          </UCard>
        </div>
      </UContainer>
    </section>

    <!-- ACTIVITY + STATS -->
    <section class="py-20 bg-gray-50 dark:bg-gray-900/40 border-y border-gray-200 dark:border-gray-800">
      <UContainer>
        <div class="grid lg:grid-cols-3 gap-8">
          
          <!-- Logs -->
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

          <!-- Stats -->
          <div class="grid grid-cols-2 gap-4">
            <UCard>
              <p class="text-sm text-gray-500">Запросов / сек</p>
              <p class="text-3xl font-bold">0</p>
            </UCard>

            <UCard>
              <p class="text-sm text-gray-500">Простой</p>
              <p class="text-3xl font-bold">0ms</p>
            </UCard>

            <UCard class="col-span-2">
              <p class="text-sm text-gray-500">Надёжность хранения</p>
              <p class="text-4xl font-bold text-primary-500">99.9%</p>
            </UCard>
          </div>

        </div>
      </UContainer>
    </section>
  </div>
</template>

<script setup lang="ts">
const features = [
  {
    title: 'Безопасность Rust',
    description: 'Бэкенд на Rust без утечек памяти и с максимальной производительностью.',
    icon: 'i-lucide-shield-check'
  },
  {
    title: 'Nuxt UI v3',
    description: 'Интерфейс на Tailwind v4 с полной поддержкой светлой и тёмной темы.',
    icon: 'i-lucide-zap'
  },
  {
    title: 'Управление данными',
    description: 'Связи между книгами и авторами с реактивной валидацией.',
    icon: 'i-lucide-database'
  }
]

const recentActions = [
  { user: '', action: '', target: '', time: '' },
]
</script>