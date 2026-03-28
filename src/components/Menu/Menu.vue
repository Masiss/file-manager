<script setup>
import { ref, defineProps, computed } from 'vue';
// import { useMenu } from './menu.js';
const props = defineProps(['x', 'y']);
const generalMenu = ref([
  {
    name: 'Open',
    action: () => openItem(),
  },
  {
    name: 'Copy',
    action: () => copy(),
  },
  {
    name: 'Cut',
    action: () => cut(),
  },
  {
    name: 'Paste',
    action: () => paste(),
  },
  {
    name: 'Properties',
    action: () => property(),
  },
  {
    name: 'Delete',
    action: () => deleteItem(),
  },
  {
    name: 'Rename',
    action: () => rename(),
  },
]);
const fileMenu = ref([
  {
    name: 'Open with ...',
    action: () => openWith(),
  },
  {
    name: 'Archive to ...',
    action: () => archive(),
  },
  {
    name: 'Extract',
    action: () => extract(),
  },
]);
const mixedMenu = ref([]);
const diskMenu = ref([]);
const selectedItems = ref([]);
watch(
  () => props.x,
  () => {
    selectedItems.value = document.querySelectorAll('[data-path].selected');
  },
);
const combineMenu = (menuType) => {
  return [...menu, ...(menuType ?? [])];
};
const getMenuType = computed(() => {
  let selectedItems = props.selectedItems.value;
  if (selectedItems.length === 1) {
    return selectedItems.dataset.type === 'File'
      ? combineMenu(fileMenu)
      : combineMenu();
  } else if (selectedItems.length > 1) {
    return combineMenu(mixedMenu);
  } else if (selectedItems.dataset.type === 'Disk') {
    return combineMenu(diskMenu);
  }
});
const handleMenu = () => {};
const handleCopy = (item) => {};
const handlePaste = (item) => {};
const handleClick = (e, action) => {};
</script>
<template>
  <div
    class="menu"
    :style="{ left: `${props.menuX}px`, top: `${props.menuY}px` }"
  >
    <div>
      <div v-for="item in menu">
        <span @click="handleClick(item.action)">{{ item.name }}</span>
      </div>
    </div>
  </div>
</template>
<style>
.menu {
  position: absolute;
  z-index: 999;
  width: fit-content;
  height: fit-content;
  display: flex;
  flex-direction: column;
}
.menu-item {
  padding: 0;
  border: 0;
}
</style>
