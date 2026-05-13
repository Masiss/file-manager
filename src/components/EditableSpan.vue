<script setup>
import { ref, nextTick, onMounted, useTemplateRef, watch } from 'vue';
import { useMenuStore } from '../store/menu.js';
const props = defineProps(['text', 'isEditing']);
const emits = defineEmits(['confirm', 'cancel']);
const menuStore = useMenuStore();
const span = useTemplateRef('editableSpan');
watch(
  () => props.isEditing,
  async (isEditing) => {
    if (!isEditing) {
      span.value?.removeEventListener('dblclick', preventDblClick);
      return;
    }
    await nextTick();
    span.value?.focus();
    span.value.addEventListener('dblclick', preventDblClick);
  },
  { immediate: true },
);
const preventDblClick = (e) => e.stopPropagation();
const onKeydown = (e) => {
  e.stopPropagation();
  if (e.key === 'Enter') {
    e.preventDefault();
    span.value.blur();
  } else if (e.key === 'Escape') {
    // span.value.innerText = props.text;
    menuStore.cancelFn();
  }
};

const onBlur = (e) => {
  const newText = e.target.innerText.trim();
  if (newText && newText !== props.text) {
    menuStore.confirmFn(span.value.innerText);
  } else {
    menuStore.cancelFn();
  }
};
</script>
<template>
  <span
    ref="editableSpan"
    class="name-span"
    :contenteditable="isEditing"
    @blur="onBlur"
    @keydown="onKeydown"
    @mousedown.stop
  >
    {{ props.text }}
  </span>
</template>
