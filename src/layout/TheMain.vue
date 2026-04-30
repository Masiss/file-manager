<script setup>
import {
  nextTick,
  onMounted,
  ref,
  shallowRef,
  useTemplateRef,
  watch,
  computed,
} from 'vue';
import { usePathStore } from '../store/path.js';
import { storeToRefs } from 'pinia';
import SideBar from '../components/SideBar.vue';
import { useDragSelect } from './dragSelect.js';
import Menu from '../components/Menu/Menu.vue';
import Icon from '../components/Icon.vue';
import Toast from '../components/Toast.vue';
import { useMenuStore } from '../store/menu.js';
import { useToastStore } from '../store/toast.js';
const toastStore = useToastStore();
const menuStore = useMenuStore();
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
    intersected.value = [];
  },
);
const { is_dragging, box, box_style, handleMouseDown, intersected } =
  useDragSelect(draggable_container);
const scrollInfo = ref({
  scrollTop: draggable_container.value?.scrollTop,
  clientHeight: draggable_container.value?.clientHeight,
  scrollHeight: draggable_container.value?.scrollHeight,
});
const handleOnScroll = (e) => {
  let { scrollTop, clientHeight, scrollHeight } = e.target;

  scrollInfo.value = { scrollTop, clientHeight, scrollHeight };
};

onMounted(() => {
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
    @contextmenu.prevent="menuStore.handleContextMenu"
    class="draggable-container layout_browser container"
    ref="draggable_container"
    id="draggable_container"
  >
    <div ref="progressbar_container" id="progressbar_container"></div>
    <component
      ref="viewComponent"
      :is="view"
      :items="items"
      :isDragging="is_dragging"
      :scrollInfo="scrollInfo"
      :intersected="intersected"
    />
    <div id="toast-list-container" class="toast-list-container">
      <Toast v-if="toastStore.isPopupToast" />
    </div>
    <div id="bottom-line-container" class="bottom-line-container"></div>
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
      v-if="menuStore.isMenuShow"
      :style="{ left: menuStore.x + 'px', top: menuStore.y + 'px' }"
    />
  </Teleport>
</template>
<style>
#progressbar_container {
  position: sticky;
  top: 0;
}
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
