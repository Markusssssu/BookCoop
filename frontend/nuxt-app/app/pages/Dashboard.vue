<template>
  <UContainer class="py-8 space-y-8">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div>
        <h1 class="text-3xl font-black italic uppercase tracking-tighter">Панель управления</h1>
        <p class="text-gray-500">Мониторинг состояния системы в 2026 году</p>
      </div>
      <div class="flex gap-2">
        <UButton icon="i-lucide-download" variant="soft" color="neutral">Отчет</UButton>
        <UButton icon="i-lucide-refresh-cw" color="primary">Обновить</UButton>
      </div>
    </div>

    <!-- Статистика -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <UCard v-for="s in stats" :key="s.label" class="relative overflow-hidden">
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
          <span class="text-gray-400">за 7 дней</span>
        </div>
      </UCard>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- ГРАФИК (2/3 ширины) -->
      <UCard class="lg:col-span-2" title="Динамика посещений">
        <template #header>
          <div class="flex items-center justify-between">
            <h3 class="font-bold">Активность читателей</h3>
            <USelectMenu :options="['7 дней', '30 дней', 'Год']" size="sm" class="w-32" />
          </div>
        </template>
        <DashboardChart />
      </UCard>

      <!-- ТАБЛИЦА/СПИСОК (1/3 ширины) -->
      <UCard title="Последние операции">
        <div class="space-y-4">
          <div v-for="order in recentOrders" :key="order.id" class="flex items-center justify-between border-b border-gray-100 dark:border-gray-800 pb-3 last:border-0">
            <div class="flex flex-col">
              <span class="text-sm font-bold">{{ order.book }}</span>
              <span class="text-xs text-gray-500">ID: {{ order.id }} • {{ order.user }}</span>
            </div>
            <UBadge 
              :color="order.status === 'Выдано' ? 'primary' : 'neutral'" 
              variant="subtle" 
              size="sm"
            >
              {{ order.status }}
            </UBadge>
          </div>
          <UButton block variant="ghost" size="sm">Все транзакции</UButton>
        </div>
      </UCard>
    </div>

    <!-- Нижняя секция: Быстрые действия -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <UCard class="bg-gray-900 text-white">
        <h4 class="font-bold mb-2">Статус API (Rust)</h4>
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
          <span class="text-sm opacity-80">Система стабильна</span>
        </div>
      </UCard>
      <!-- Сюда можно добавить еще виджеты -->
    </div>
  </UContainer>
</template>

<script setup lang="ts">
const stats = [
  { label: 'Книг в базе', value: '1,240', delta: '+12%', icon: 'i-lucide-library' },
  { label: 'Активных авторов', value: '48', delta: '+2', icon: 'i-lucide-users' },
  { label: 'Выдано сегодня', value: '85', delta: '-5%', icon: 'i-lucide-book-up' },
  { label: 'Загрузка сервера', value: '14ms', delta: 'Stable', icon: 'i-lucide-cpu' }
]

const recentOrders = [
  { id: '#3401', book: 'Rust for C++ Devs', user: 'alex_dev', status: 'Выдано' },
  { id: '#3402', book: 'Clean Architecture', user: 'mariya_r', status: 'Возврат' },
  { id: '#3403', book: 'The Great Gatsby', user: 'ivan_00', status: 'Ожидание' },
]
</script>
