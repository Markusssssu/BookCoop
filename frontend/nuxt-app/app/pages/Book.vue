<template>
  <UContainer class="py-8">
    <div class="flex justify-between items-center mb-6">
      <h1 class="text-3xl font-bold">Каталог книг</h1>
      <!-- Кнопка открывает форму в модалке -->
      <UModal title="Новая книга">
        <UButton label="Добавить книгу" icon="i-lucide-plus" />
        <template #body>
          <BookForm />
        </template>
      </UModal>
    </div>

    <!-- Сетка книг -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      <UCard v-for="i in 6" :key="i" class="hover:ring-2 ring-primary-500 transition-all">
        <div class="flex gap-4">
          <div class="w-24 h-32 bg-gray-200 dark:bg-gray-800 rounded flex items-center justify-center">
            <UIcon name="i-lucide-book-image" class="text-3xl opacity-20" />
          </div>
          <div class="flex-1">
            <h3 class="font-bold text-lg">Название книги #{{ i }}</h3>
            <p class="text-sm text-gray-500">Автор Авторович</p>
            <UBadge label="Фантастика" variant="subtle" size="sm" class="mt-2" />
          </div>
        </div>
        <template #footer>
          <div class="flex justify-end gap-2">
            <UButton icon="i-lucide-pencil" variant="ghost" color="neutral" size="sm" />
            <UButton icon="i-lucide-trash" variant="ghost" color="error" size="sm" />
          </div>
        </template>
      </UCard>
    </div>
  </UContainer>
</template>