<script setup>
import { computed, onMounted, useTemplateRef } from 'vue';
const props = defineProps([
  'itemsLength',
  'loadedLength',
  'isDragging',
  'isFinding',
  'selectedLength',
]);

const loadedItems = computed(() => {
  // let total = sorted_items.value.length;
  // let loaded = displaying_items.value.length;
  return `${props.itemsLength} items / ${props.loadedLength} items`;
});
const state = computed(() => {
  return props.isDragging
    ? 'is dragging'
    : props.isFinding
      ? 'is finding'
      : props.selectedLength > 0
        ? `selected ${props.selectedLength} items`
        : '';
});
const bottomLine = useTemplateRef('bottomLine');
const setBottomLineWidth = () => {
  let containerWidth = bottomLine.value.parentElement.parentElement.clientWidth;
  bottomLine.value.style.width = containerWidth + 'px';
};
onMounted(() => {
  setBottomLineWidth();
});
</script>
<template>
  <div ref="bottomLine" class="bottom-line">
    <div>{{ loadedItems }}</div>
    <div>{{ state }}</div>
  </div>
</template>
<style scoped>
.bottom-line {
  width: 100%;
  display: inline-flex;
  justify-content: space-between;
  div {
    padding: 0 15px;
  }
}
</style>
