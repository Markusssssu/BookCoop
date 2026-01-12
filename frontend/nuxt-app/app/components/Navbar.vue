<script setup lang="ts">
const isOpen = ref(false)
const authCookie = useCookie('auth_token')

const adminName = "Mark Mansurov Radickovich"

const navItems = [
  { label: 'Главная', to: '/', icon: 'i-lucide-home' },
  { label: 'Книги', to: '/book', icon: 'i-lucide-book' },
  { label: 'Авторы', to: '/author', icon: 'i-lucide-users' },
  { label: 'Панель', to: '/dashboard', icon: 'i-lucide-layout-dashboard' }
]

// Функция выхода
const handleLogout = async () => {
  authCookie.value = null
  await navigateTo('/login')
}
</script>

<template>
  <header class="border-b border-gray-200 dark:border-gray-800 bg-white/80 dark:bg-gray-950/80 backdrop-blur-md sticky top-0 z-50">
    <UContainer>
      <div class="flex items-center justify-between h-16">
        
        <!-- Логотип и Десктопное меню -->
        <div class="flex items-center gap-8">
          <NuxtLink to="/" class="flex items-center gap-2 group">
            <div class="w-8 h-8 bg-primary-500 rounded-lg flex items-center justify-center group-hover:rotate-12 transition-transform">
              <UIcon name="i-lucide-library" class="text-white text-xl" />
            </div>
            <span class="font-black text-xl tracking-tighter uppercase">Blazing<span class="text-primary-500">Lib</span></span>
          </NuxtLink>

          <UNavigationMenu :items="navItems" class="hidden md:flex" />
        </div>

        <!-- Правая часть: Пользователь и Кнопки -->
        <div class="flex items-center gap-3">
          <!-- Информация об админе (только десктоп) -->
          <div class="hidden lg:flex flex-col items-end mr-2">
            <span class="text-sm font-bold leading-none">{{ adminName }}</span>
            <span class="text-[10px] text-primary-500 uppercase font-black tracking-widest">Main Admin</span>
          </div>

          <UColorModeButton />

          <!-- Кнопка выхода -->
          <UButton
            icon="i-lucide-log-out"
            color="neutral"
            variant="ghost"
            class="hidden md:flex"
            @click="handleLogout"
          />

          <!-- Бургер для мобилок -->
          <UButton
            icon="i-lucide-menu"
            color="neutral"
            variant="ghost"
            class="md:hidden"
            @click="isOpen = true"
          />
        </div>
      </div>
    </UContainer>

    <!-- Мобильное меню (Slideover) -->
    <USlideover v-model:open="isOpen" title="Меню управления">
      <template #body>
        <div class="flex flex-col h-full justify-between py-4">
          <div class="space-y-6">
            <!-- Профиль в мобильном меню -->
            <div class="p-4 bg-gray-50 dark:bg-gray-900 rounded-xl flex items-center gap-3">
              <UAvatar :alt="adminName" size="lg" />
              <div>
                <p class="font-bold text-sm">{{ adminName }}</p>
                <p class="text-xs text-gray-500">Библиотекарь</p>
              </div>
            </div>

            <UNavigationMenu 
              :items="navItems" 
              orientation="vertical" 
              class="w-full"
              @click="isOpen = false" 
            />
          </div>

          <UButton 
            label="Выйти из системы" 
            icon="i-lucide-log-out" 
            color="error" 
            variant="soft" 
            block 
            @click="handleLogout"
          />
        </div>
      </template>
    </USlideover>
  </header>
</template>

<style scoped>
:deep(.router-link-active) {
  color: var(--ui-primary);
}
</style>
