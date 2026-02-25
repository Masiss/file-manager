<script setup>
import { ref, defineProps } from 'vue';
const props = defineProps(['menuX', 'menuY', 'selectingItems']);
let menu = ref([
  {
    name: 'Copy',
    action: () => handleCopy(),
  },
  {
    name: 'Paste',
    action: () => handlePaste(),
  },
]);
const handleCopy = (item) => {};
const handlePaste = (item) => {};
const handleClick = (e, action) => {
  e.preventDefault();
  action(props.selectingItems?.value);
};
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
