<template>
  <UContainer class="py-8">
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Левая колонка: Форма -->
      <div class="lg:col-span-1">
        <h2 class="text-2xl font-bold mb-4">Регистрация</h2>
        <AuthorForm />
      </div>

      <!-- Правая колонка: Список -->
      <div class="lg:col-span-2">
        <h2 class="text-2xl font-bold mb-4">Все авторы</h2>
        <UCard>
          <div class="divide-y divide-gray-200 dark:divide-gray-800">
            <div v-for="author in []" 
                 :key="author" 
                 class="py-4 flex items-center justify-between">
              <div class="flex items-center gap-3">
                <UAvatar :alt="author" size="lg" />
                <div>
                  <div class="font-medium">{{ author }}</div>
                  <div class="text-sm text-gray-500">0 написанных книг</div>
                </div>
              </div>
              <UButton label="Профиль" variant="outline" size="sm" />
            </div>
          </div>
        </UCard>
      </div>
    </div>
  </UContainer>
</template>