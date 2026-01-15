<script setup lang="ts">
// Создаем обычный массив со статическими данными
const authorsList = ref([
  {
    id: 1,
    name: "Джордж Оруэлл",
    birth_date: "25.06.1903",
    description: "Писатель, он очень хороший человек",
    books_count: 5
  },
  {
    id: 2,
    name: "Лев Толстой",
    birth_date: "09.09.1828",
    description: "Классик мировой литературы",
    books_count: 12
  }
])

// Заглушки для функций, чтобы шаблон не выдавал ошибки
const pending = ref(false)
const refresh = () => console.log('Данные обновлены (статично)')
</script>

<template>
  <UContainer class="py-8">
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <!-- Левая колонка: Форма -->
      <div class="lg:col-span-1">
        <h2 class="text-2xl font-bold mb-4">Регистрация</h2>
        <AuthorForm @success="refresh" />
      </div>

      <!-- Правая колонка: Список -->
      <div class="lg:col-span-2">
        <h2 class="text-2xl font-bold mb-4">Все авторы</h2>

        <UCard>
          <UScrollArea class="h-[500px] px-4">
            <div class="divide-y divide-gray-200 dark:divide-gray-800">
              <div v-for="author in authorsList"
                   :key="author.id"
                   class="py-4 flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <UAvatar
                      :alt="author.name"
                      size="lg"
                      background="bg-primary-500"
                  />
                  <div>
                    <div class="font-medium text-gray-900 dark:text-white">
                      {{ author.name }}
                    </div>
                    <div class="text-sm text-gray-500">
                      {{ author.birth_date }} • {{ author.description }}
                    </div>
                    <div class="text-xs text-primary-500 mt-1">
                      Книг в базе: {{ author.books_count }}
                    </div>
                  </div>
                </div>
                <UButton label="Профиль" variant="outline" size="sm" color="neutral" />
              </div>
            </div>
          </UScrollArea>
        </UCard>
      </div>
    </div>
  </UContainer>
</template>
