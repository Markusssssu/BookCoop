<template>
  <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
    <div>
      <h1 class="text-3xl font-black italic uppercase tracking-tighter">
        Панель управления
      </h1>
      <p class="text-gray-500">Мониторинг состояния системы в 2026 году</p>
    </div>

    <div class="flex gap-2">
      <UButton icon="i-lucide-download" variant="soft" color="neutral">
        Отчет
      </UButton>
      <UButton icon="i-lucide-refresh-cw" color="primary">
        Обновить
      </UButton>
    </div>
  </div>
</template>
