<template>
  <UModal title="Новая книга" v-model="model">
    <template #content>
      <div class="p-4">
        <BookForm @success="onSuccess" />
      </div>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import BookForm from './BookForm.vue'

const model = defineModel<boolean>()
const emit = defineEmits(['created'])

function onSuccess() {
  model.value = false
  emit('created')
}
</script>
