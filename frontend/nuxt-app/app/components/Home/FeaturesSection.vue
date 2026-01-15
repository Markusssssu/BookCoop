<template>
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
</template>

<script setup lang="ts">
const features = [
  {
    title: 'Управление книгами',
    description: 'Создавайте, редактируйте и удаляйте книги мгновенно с поддержкой Nuxt UI.',
    icon: 'i-lucide-book'
  },
  {
    title: 'Авторы и связи',
    description: 'Следите за авторами, связями с книгами и статистикой по базе.',
    icon: 'i-lucide-users'
  },
  {
    title: 'Реальная аналитика',
    description: 'Просматривайте активность пользователей и отчёты по библиотеке.',
    icon: 'i-lucide-bar-chart-3'
  }
]
</script>
