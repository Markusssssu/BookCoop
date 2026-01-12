<script setup lang="ts">
const navItems = [
  { label: 'Главная', to: '/', icon: 'i-lucide-home' },
  { label: 'Книги', to: '/book', icon: 'i-lucide-book' },
  { label: 'Авторы', to: '/author', icon: 'i-lucide-users' },
  { label: 'Панель', to: '/dashboard', icon: 'i-lucide-layout-dashboard' }
]

const isOpen = ref(false) 
</script>

<template>
  <header class="border-b border-gray-200 dark:border-gray-800 p-4">
    <div class="container mx-auto flex items-center justify-between">
      
      <div class="font-bold text-xl">BookCoop</div>

      <UNavigationMenu :items="navItems" class="hidden md:flex" />

      <div class="flex items-center gap-2">
        <UColorModeButton />

        <UButton
          icon="i-lucide-menu"
          color="neutral"
          variant="ghost"
          class="md:hidden"
          @click="isOpen = true"
        />
      </div>
    </div>

    <USlideover v-model:open="isOpen" title="Навигация">
      <template #body>
        <UNavigationMenu 
          :items="navItems" 
          orientation="vertical" 
          class="w-full"
          @click="isOpen = false" 
        />
      </template>
    </USlideover>
  </header>
</template>
