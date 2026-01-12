<script setup lang="ts">
const features = [
  {
    title: 'Безопасность Rust',
    description: 'Наш бэкенд на Rust гарантирует отсутствие утечек памяти и максимальную скорость обработки запросов.',
    icon: 'i-lucide-shield-check'
  },
  {
    title: 'Nuxt UI v3',
    description: 'Современный интерфейс на базе Tailwind v4, оптимизированный для работы в темной и светлой темах.',
    icon: 'i-lucide-zap'
  },
  {
    title: 'Управление данными',
    description: 'Гибкая система связей между книгами и авторами с мгновенной валидацией на лету.',
    icon: 'i-lucide-database'
  }
]

const recentActions = [
  { user: 'Admin', action: 'добавил книгу', target: '«Путь к Rust»', time: '2 мин. назад' },
  { user: 'System', action: 'обновила автора', target: 'Лев Толстой', time: '15 мин. назад' },
  { user: 'Editor', action: 'удалил дубликат', target: 'ID: 442', time: '1 час назад' }
]
</script>

<template>
  <div class="space-y-20 pb-20">
    <!-- 1. HERO SECTION -->
    <section class="relative overflow-hidden pt-16 pb-12 lg:pt-24">
      <UContainer>
        <div class="text-center space-y-8 relative z-10">
          <UBadge 
            variant="soft" 
            color="primary" 
            size="lg"
            class="animate-bounce"
          >
            Powered by Rust & Nuxt
          </UBadge>
          
          <h1 class="text-5xl lg:text-7xl font-black tracking-tight text-gray-900 dark:text-white">
            Библиотека в <span class="text-primary-500">реальном времени</span>
          </h1>
          
          <p class="max-w-3xl mx-auto text-xl text-gray-500 dark:text-gray-400">
            Система управления контентом нового поколения. Контролируйте вашу базу данных через 
            реактивный интерфейс с мгновенным откликом.
          </p>

          <div class="flex flex-wrap justify-center gap-4">
            <UButton to="/book" size="xl" icon="i-lucide-plus-circle" class="px-8">Начать работу</UButton>
            <UButton to="/dashboard" variant="soft" color="neutral" size="xl" icon="i-lucide-bar-chart-3">Аналитика</UButton>
          </div>
        </div>

        <!-- Декоративный элемент фона -->
        <div class="absolute top-0 left-1/2 -translate-x-1/2 w-full h-full -z-0 opacity-10 pointer-events-none">
          <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-primary-500 blur-[120px] rounded-full"></div>
        </div>
      </UContainer>
    </section>

    <!-- 2. FEATURES SECTION -->
    <section>
      <UContainer>
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
          <UCard v-for="item in features" :key="item.title" class="group hover:-translate-y-2 transition-transform duration-300">
            <div class="p-2">
              <div class="w-12 h-12 rounded-lg bg-primary-500/10 flex items-center justify-center mb-4 group-hover:bg-primary-500 transition-colors">
                <UIcon :name="item.icon" class="text-2xl text-primary-500 group-hover:text-white" />
              </div>
              <h3 class="text-xl font-bold mb-2">{{ item.title }}</h3>
              <p class="text-gray-500 dark:text-gray-400">{{ item.description }}</p>
            </div>
          </UCard>
        </div>
      </UContainer>
    </section>

    <!-- 3. CONTENT SECTION (Stats + Activity) -->
    <section class="bg-gray-50 dark:bg-gray-900/50 py-16 border-y border-gray-200 dark:border-gray-800">
      <UContainer>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div class="space-y-6">
            <h2 class="text-3xl font-bold">Активность системы</h2>
            <p class="text-gray-500">Последние действия в базе данных отображаются здесь. Система работает на WebSockets для мгновенного обновления.</p>
            
            <div class="space-y-3">
              <div v-for="log in recentActions" :key="log.time" 
                class="flex items-center justify-between p-3 bg-white dark:bg-gray-800 rounded-lg border border-gray-100 dark:border-gray-700 shadow-sm"
              >
                <div class="flex items-center gap-3">
                  <UAvatar :alt="log.user" size="sm" />
                  <span class="text-sm">
                    <b class="text-primary-500">{{ log.user }}</b> {{ log.action }} <b>{{ log.target }}</b>
                  </span>
                </div>
                <span class="text-xs text-gray-400">{{ log.time }}</span>
              </div>
            </div>
            
            <UButton variant="link" icon="i-lucide-arrow-right" trailing>Смотреть все логи</UButton>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="p-8 bg-primary-500 rounded-3xl text-white space-y-2">
              <p class="text-4xl font-black">48k</p>
              <p class="text-sm opacity-80">Запросов/сек</p>
            </div>
            <div class="p-8 bg-gray-800 rounded-3xl text-white space-y-2">
              <p class="text-4xl font-black">0ms</p>
              <p class="text-sm opacity-80">Время простоя</p>
            </div>
            <div class="col-span-2 p-8 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-3xl space-y-2 text-center">
              <p class="text-5xl font-black text-primary-500">99.9%</p>
              <p class="text-sm text-gray-500 uppercase tracking-widest font-bold">Надежность хранения данных</p>
            </div>
          </div>
        </div>
      </UContainer>
    </section>

    <!-- 4. CALL TO ACTION -->
    <section>
      <UContainer>
        <UCard class="bg-primary-600 text-white overflow-hidden relative">
          <div class="relative z-10 py-8 px-4 text-center space-y-6">
            <h2 class="text-3xl font-bold">Готовы расширить свою библиотеку?</h2>
            <p class="max-w-xl mx-auto opacity-90">
              Начните добавлять авторов и книги прямо сейчас. Все изменения будут синхронизированы с бэкендом автоматически.
            </p>
            <div class="flex justify-center gap-4">
              <UButton color="neutral" variant="white" size="lg" to="/author">Добавить автора</UButton>
              <UButton color="neutral" variant="ghost" size="lg" to="/book" class="text-white ring-1 ring-white/30">Книги</UButton>
            </div>
          </div>
          <!-- Фоновый паттерн -->
          <div class="absolute inset-0 opacity-10 i-lucide-grid transform scale-150 rotate-12"></div>
        </UCard>
      </UContainer>
    </section>
  </div>
</template>
