<script setup>
import { ref, shallowRef, useTemplateRef, watch } from 'vue';
import { usePathStore } from '../store/path.js';
import { storeToRefs } from 'pinia';
import SideBar from '../components/SideBar.vue';
import { useDragSelect } from './dragSelect.js';
const store = usePathStore();
const items = ref([]);
const view = shallowRef('');
const draggable_container = useTemplateRef('draggable_container');
const { getView } = storeToRefs(store);
watch(
  () => store.items,
  () => {
    items.value = store.items;
    view.value = getView.value;
  },
);
const {
  is_dragging,
  box,
  box_style,
  scrollInfo,
  handleMouseDown,
  handleMouseMove,
  handleMouseUp,
  handleOnScroll,
} = useDragSelect(draggable_container);
</script>
<template>
  <SideBar :items="items"></SideBar>
  <!-- <button width="100%" type="button" @click="store.$reset()">Reset</button> -->
  <div
    @scroll.passive="handleOnScroll"
    @mousedown="handleMouseDown"
    @mouseup="handleMouseUp"
    @mousemove="handleMouseMove"
    class="draggable-container layout-browser"
    ref="draggable_container"
    id="draggable_container"
  >
    <component
      class="container"
      :is="view"
      :items="items"
      :isDragging="is_dragging"
      :scrollInfo="scrollInfo"
    />
  </div>
  <Teleport to="body">
    <div v-if="is_dragging" :style="box_style" ref="box" class="drag-box"></div>
  </Teleport>
</template>
<style>
.drag-box {
  position: absolute;
  background-color: blue;
  opacity: 0.5;
  z-index: 1000;
  pointer-events: none;
}
.draggable-container {
  scroll-behavior: smooth;
  position: relative;
  overflow: auto;
  user-select: none;
}
</style>
