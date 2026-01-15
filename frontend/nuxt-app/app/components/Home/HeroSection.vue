<template>
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
</template>
