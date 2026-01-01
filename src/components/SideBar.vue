<script setup>
import { ref } from 'vue';
const sidebar_items = ref([
  {
    name: 'D:\\',
    path: 'D:\\',
  },
  {
    name: 'C:\\',
    path: 'C:\\',
  },
]);
</script>
<template>
  <aside class="container sidebar">
    <ul v-for="item in sidebar_items">
      <li>{{ item.name }}</li>
    </ul>
  </aside>
</template>
