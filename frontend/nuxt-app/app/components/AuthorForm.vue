<template>
  <UCard class="w-full max-w-lg mx-auto border-dashed border-2">
    <template #header>
      <h3 class="text-xl font-semibold text-primary">Карточка автора</h3>
    </template>

    <UForm :state="state" @submit="onSubmit" class="space-y-4">
      <UFormField label="Полное имя" name="name" required>
        <UInput v-model="state.name" icon="i-lucide-user" />
      </UFormField>

      <UFormField label="Дата рождения" name="birthDate">
        <UInput v-model="state.birthDate" type="date" />
      </UFormField>

      <UFormField label="Биография" name="bio">
        <UTextarea v-model="state.bio" placeholder="Краткая информация..." autoresize />
      </UFormField>

      <UButton type="submit" variant="soft" color="secondary" block>
        Зарегистрировать автора
      </UButton>
    </UForm>
  </UCard>
</template>

<script setup lang="ts">
const state = reactive({
  name: '',
  birthDate: '',
  bio: ''
})

async function onSubmit(event: any) {
  console.log('Данные автора:', event.data)
}
</script>