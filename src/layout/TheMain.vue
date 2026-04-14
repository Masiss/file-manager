<script setup>
import {
  nextTick,
  onMounted,
  ref,
  shallowRef,
  useTemplateRef,
  watch,
} from 'vue';
import { usePathStore } from '../store/path.js';
import { storeToRefs } from 'pinia';
import SideBar from '../components/SideBar.vue';
import { useDragSelect } from './dragSelect.js';
import Menu from '../components/Menu/Menu.vue';
const pathStore = usePathStore();
const items = ref([]);
const view = shallowRef('');
const draggable_container = useTemplateRef('draggable_container');
const { getView } = storeToRefs(pathStore);
watch(
  () => pathStore.items,
  () => {
    items.value = pathStore.items;
    view.value = getView.value;
  },
);
const {
  is_dragging,
  box,
  box_style,
  handleMouseDown,
  handleMouseMove,
  handleMouseUp,
  intersected,
  handleClick,
} = useDragSelect(draggable_container);
const scrollInfo = ref({
  scrollTop: draggable_container.value?.scrollTop,
  clientHeight: draggable_container.value?.clientHeight,
  scrollHeight: draggable_container.value?.scrollHeight,
});
const handleOnScroll = (e) => {
  let { scrollTop, clientHeight, scrollHeight } = e.target;

  scrollInfo.value = { scrollTop, clientHeight, scrollHeight };
};
const isShowMenu = ref(false);
const x = ref();
const y = ref();
const handleContextMenu = (e) => {
  let hasSelected = intersected.value.length > 0;
  let closestRow = e.target.closest('tr');
  let isSelected = closestRow.classList.contains('selected');
  if (!isSelected) {
    if (hasSelected) {
      intersected.value.forEach((tr) => tr.classList.remove('selected'));
      intersected.value = [];
    }
    closestRow.classList.add('selected');
    intersected.value.push(closestRow);
  }
  x.value = e.clientX;
  y.value = e.clientY;
  isShowMenu.value = true;
  nextTick(() => {
    window.addEventListener('click', closeContextMenu);
  });
};
const closeContextMenu = (e) => {
  isShowMenu.value = false;
};
onMounted(() => {
  console.log(items.value);
  scrollInfo.value = {
    scrollTop: draggable_container.value?.scrollTop,
    clientHeight: draggable_container.value?.clientHeight,
    scrollHeight: draggable_container.value?.scrollHeight,
  };
});
</script>
<template>
  <SideBar :items="items"></SideBar>
  <!-- <button width="100%" type="button" @click="store.$reset()">Reset</button> -->
  <div
    @scroll.passive="handleOnScroll"
    @mousedown="handleMouseDown"
    @mouseup="handleMouseUp"
    @mousemove="handleMouseMove"
    @contextmenu.prevent="handleContextMenu"
    @click="handleClick"
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
      :intersected="intersected"
    />
  </div>
  <Teleport to="body">
    <div
      v-show="is_dragging"
      :style="box_style"
      ref="box"
      class="drag-box"
    ></div>
  </Teleport>
  <Teleport to="body">
    <Menu
      class="menu"
      v-show="isShowMenu"
      :style="{ left: x + 'px', top: y + 'px' }"
      :selectedItems="intersected"
    />
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
  display: block;
  scroll-behavior: smooth;
  position: relative;
  overflow: auto;
  user-select: none;
  padding: 0 1rem 0 0;
}
.menu {
  position: absolute;
  z-index: 999;
  width: fit-content;
  height: fit-content;
  display: flex;
  flex-direction: column;
}
</style>
