<script setup>
import { useToastStore } from '../store/toast.js';
const toastStore = useToastStore();
</script>
<template>
  <div class="toast-list">
    <component
      v-for="toast in toastStore.toast_list"
      :is="toast.toast_type"
      :data="toast.data"
    />
  </div>
</template>
<style scoped></style>
