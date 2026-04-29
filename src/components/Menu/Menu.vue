<script setup>
import { ref, watch, defineProps } from 'vue';
import { useMenuStore } from '../../store/menu.js';
const menuStore = useMenuStore();
</script>
<template>
  <div id="menu" class="container">
    <ul>
      <li class="menu-item" v-for="item in menuStore.menu">
        <span @click="menuStore.handleClick(item.action)">{{ item.name }}</span>
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
