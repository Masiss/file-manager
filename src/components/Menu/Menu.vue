<script setup>
import { ref, watch, defineProps, useTemplateRef, onMounted } from 'vue';
import { useMenuStore } from '../../store/menu.js';
const menuStore = useMenuStore();
const currentSub = ref(null);
const menuRef = useTemplateRef('menuRef');
watch(menuRef, () => (menuStore.menuRef = menuRef.value));
</script>
<template>
  <div ref="menuRef" id="menu" class="container">
    <ul>
      <li
        class="menu-item"
        v-for="(item, index) in menuStore.menu"
        :key="index"
        @mouseover.prevent="currentSub = index"
      >
        <span @click.prevent="menuStore.handleClick(item.action)">{{
          item.name
        }}</span>
        <ul v-show="currentSub === index" class="sub-menu">
          <li v-for="sub in item.subs">
            <span @click.prevent="sub.action">{{ sub.name }}</span>
          </li>
        </ul>
      </li>
    </ul>
  </div>
</template>
<style scoped>
.sub-menu {
  position: absolute;
  left: 100%;
  top: 0;
  width: 150px;
}
.menu-item {
  padding: 2px 10px;
  position: relative;
}
.container {
  border: none;
  border-radius: 0;
}
</style>
