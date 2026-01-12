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

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-950 p-4">
    <UCard class="w-full max-w-md">
      <template #header>
        <div class="text-center">
          <UIcon name="i-lucide-lock" class="text-4xl text-primary mb-2" />
          <h1 class="text-2xl font-bold">Вход в систему</h1>
          <p class="text-sm text-gray-500">Только для персонала библиотеки</p>
        </div>
      </template>

      <UForm :state="authData" @submit="handleLogin" class="space-y-4">
        <UFormField label="Полное имя администратора" name="fullName">
          <UInput v-model="authData.fullName" icon="i-lucide-user" placeholder="Имя Фамилия Отчество" />
        </UFormField>

        <UFormField label="Ключевое слово" name="keyword">
          <UInput v-model="authData.keyword" type="password" icon="i-lucide-key" placeholder="••••••••" />
        </UFormField>

        <UButton type="submit" block :loading="isLoading">
          Авторизоваться
        </UButton>
      </UForm>

      <template #footer>
        <p class="text-center text-xs text-gray-400 uppercase tracking-widest">
          Secure Access 2026
        </p>
      </template>
    </UCard>
  </div>
</template>
