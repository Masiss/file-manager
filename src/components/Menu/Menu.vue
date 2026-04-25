<script setup>
import { ref, watch, defineProps, computed, toRef, onMounted } from 'vue';
import { useMenu } from './menu.js';
const props = defineProps(['selectedItems']);
const { menu, handleClick } = useMenu(toRef(props, 'selectedItems'));
</script>
<template>
  <div class="container">
    <ul>
      <li class="menu-item" v-for="item in menu">
        <span @click="handleClick(item.action)">{{ item.name }}</span>
      </li>
    </ul>
  </div>
</template>
<style scoped>
.menu-item {
  padding: 2px 10px;
}
.container {
  border: none;
  border-radius: 0;
}
</style>
