
<template>
  <div
    class="min-h-screen w-full flex items-center justify-center bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-900 dark:to-gray-950"
  >
    <UCard
      class="w-full max-w-md backdrop-blur bg-white/80 dark:bg-gray-900/70 shadow-xl border border-gray-200 dark:border-gray-800"
    >
      <template #header>
        <div class="flex flex-col items-center text-center space-y-3 py-2">
          <div class="w-14 h-14 flex items-center justify-center rounded-full bg-primary/10">
            <UIcon name="i-lucide-lock" class="text-primary text-3xl" />
          </div>

          <h1 class="text-2xl font-semibold tracking-tight">
            Вход в систему
          </h1>

          <p class="text-sm text-gray-500">
            Панель администрирования библиотеки
          </p>
        </div>
      </template>

      <UForm :state="authData" @submit="handleLogin" class="space-y-5">
        <UFormField label="Полное имя" name="fullName">
          <UInput
            v-model="authData.fullName"
            icon="i-lucide-user"
            size="lg"
            placeholder="Имя Фамилия Отчество"
            class="w-full"
          />
        </UFormField>

        <UFormField label="Ключ доступа" name="keyword">
          <UInput
            v-model="authData.keyword"
            type="password"
            icon="i-lucide-key"
            size="lg"
            placeholder="Введите ключ"
            class="w-full"
          />
        </UFormField>

        <UButton
          type="submit"
          block
          size="lg"
          color="primary"
          :loading="isLoading"
          class="mt-2"
        >
          Авторизоваться
        </UButton>
      </UForm>

      <template #footer>
        <div class="text-center text-xs text-gray-400 tracking-widest">
          SECURE ACCESS · 2026
        </div>
      </template>
    </UCard>
  </div>
</template>

<script setup lang="ts">

definePageMeta({
  layout: false
})

const authData = reactive({
  fullName: '',
  keyword: ''
})

const isLoading = ref(false)
const toast = useToast()

async function handleLogin() {
  isLoading.value = true
  
  if (
    authData.fullName === 'Mark Mansurov Radickovich' && 
    authData.keyword === 'BLAZING FAST'
  ) {
    // Сохраняем "токен" в Cookie (живет 1 день)
    const authCookie = useCookie('auth_token')
    authCookie.value = 'authenticated_admin'
    
    toast.add({ title: 'Доступ разрешен', color: 'primary' })
    await navigateTo('/')
  } else {
    toast.add({ title: 'Доступ запрещен', description: 'Неверные данные', color: 'error' })
  }
  
  isLoading.value = false
}
</script>
