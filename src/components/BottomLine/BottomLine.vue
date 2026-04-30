<script setup>
import { computed, onMounted, useTemplateRef, ref } from 'vue';
import Icon from '../Icon.vue';
import Toast from '../Toast.vue';
import { useToastStore } from '../../store/toast.js';
const toastStore = useToastStore();
const props = defineProps([
  'itemsLength',
  'loadedLength',
  'isDragging',
  'isFinding',
  'selectedLength',
]);

const loadedItems = computed(() => {
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
const isShowToastBox = ref(false);
const icon = computed(() =>
  isShowToastBox.value ? 'chevron-down' : 'chevron-up',
);
const showToastBox = () => {
  isShowToastBox.value = !isShowToastBox.value;
  toastStore.isShowAll = isShowToastBox.value;
};
</script>
<template>
  <div ref="bottomLine" class="bottom-line">
    <div class="bottom-line__left">{{ loadedItems }}</div>
    <div class="bottom-line__right">
      <span>
        {{ state }}
      </span>
      <button @click.prevent="showToastBox">
        <Icon :icon="icon" />
      </button>
    </div>
    <div
      v-if="isShowToastBox"
      class="container"
      style="
        position: absolute;
        bottom: 2.5rem;
        right: 1.5rem;
        padding: 0;
        width: 30%;
        max-width: 300px;
      "
    >
      <Toast />
    </div>
  </div>
</template>
<style scoped>
.bottom-line {
  width: 100%;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  > div {
    padding: 0 15px;
  }
}
.bottom-line__right {
  align-items: center;
  button {
    padding: 0;
    border: 0;
  }
}
</style>
