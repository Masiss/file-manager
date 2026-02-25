<script setup>
import { ref, onMounted, provide, watch, nextTick } from 'vue';
const is_dragging = ref(false);
const draggable_container = ref(null);
const draggable_container_rect = ref(null);
const box = ref(null);
const box_style = ref({});
let box_start = { x: 0, y: 0 };
let box_end = { x: 0, y: 0 };
let start_scroll_top = 0;

const handleMouseDown = (e) => {
  is_dragging.value = true;
  box_start = { x: e.clientX, y: e.clientY };

  box_style.value.left = box_start.x + 'px';
  box_style.value.top = box_start.y + 'px';
  start_scroll_top = draggable_container.value?.scrollTop || 0;
};
const scroll_speed = 15;
const edge_threshold = 30;
const handleMouseMove = (e) => {
  if (!is_dragging.value) return;

  const current_scroll = draggable_container.value.scrollTop || 0;

  box_end = { x: e.clientX, y: e.clientY };

  const containerRect = draggable_container.value.getBoundingClientRect();

  const start_content_y = box_start.y - containerRect.top + start_scroll_top;

  const end_content_y = box_end.y - containerRect.top + current_scroll;

  const top_content = Math.min(start_content_y, end_content_y);
  const bottom_content = Math.max(start_content_y, end_content_y);
  const height_content = bottom_content - top_content;

  const top_viewport = top_content - current_scroll + containerRect.top;

  let minX = Math.min(box_start.x, box_end.x);
  let new_width = Math.abs(box_start.x - box_end.x);

  box_style.value = {
    left: minX + 'px',
    top: top_viewport + 'px',
    width: new_width + 'px',
    height: height_content + 'px',
  };

  // Auto-scroll
  const rect = draggable_container.value.getBoundingClientRect();
  const mouseY = e.clientY;

  if (mouseY > rect.bottom - edge_threshold) {
    draggable_container.value.scrollBy({
      top: scroll_speed,
      behavior: 'instant',
    });
  } else if (mouseY < rect.top + edge_threshold) {
    draggable_container.value.scrollBy({
      top: -scroll_speed,
      behavior: 'instant',
    });
  }

  intersections();
};
const handleMouseUp = (e) => {
  is_dragging.value = false;
  box_style.value = {};
  box_start = { x: 0, y: 0 };
  box_end = { x: 0, y: 0 };
};

const registeredItem = ref([]);
const register = (...items) => {
  items.forEach((item) => {
    registeredItem.value.push(item);
  });
};
const intersections = () => {
  let intersected = [];

  let boxRect = box.value.getBoundingClientRect();
  registeredItem.value.forEach((el) => {
    const rect = el.getBoundingClientRect();
    const isIntersected =
      boxRect.left < rect.right &&
      boxRect.left + boxRect.width > rect.left &&
      boxRect.top < rect.bottom &&
      boxRect.top + boxRect.height > rect.top;

    if (isIntersected) {
      el.classList.add('selected');
      intersected.push(el);
    } else {
      if (el.classList.contains('selected')) el.classList.remove('selected');
    }
  });
};
onMounted(() => {
  draggable_container_rect.value =
    draggable_container.value.getBoundingClientRect();
});
provide('itemsRect', {
  register,
});
</script>
<template>
  <div
    @mousedown="handleMouseDown"
    @mouseup="handleMouseUp"
    @mousemove="handleMouseMove"
    class="draggable-container"
    ref="draggable_container"
  >
    <slot />
  </div>
  <div v-if="is_dragging" :style="box_style" ref="box" class="drag-box"></div>
</template>
<style>
.view-wrapper {
  border: 0;
  width: 100%;
  height: 100%;
}
.drag-box {
  position: absolute;
  background-color: blue;
  opacity: 0.5;
  z-index: 1000;
  pointer-events: none;
}
.draggable-container {
  position: relative;
  overflow: auto;
  user-select: none;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>
