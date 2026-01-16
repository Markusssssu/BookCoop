<script setup lang="ts">
import BookForm from './BookForm.vue'

const model = defineModel<boolean>()
const emit = defineEmits(['created'])

function onSuccess() {
  model.value = false       // ← закрываем модалку
  emit('created')           // ← говорим родителю обновиться
}
</script>

<template>
  <UModal v-model="model" title="Новая книга">
    <template #content>
      <div class="p-4">
        <BookForm @success="onSuccess" />
      </div>
    </template>
  </UModal>
</template>
