import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
export const useContextMenuStore = defineStore('context-menu', () => {
  const copyingFile = ref([]);
  const selectingFile = ref([]);
  const isSelecting = ref(false);
  const menuType = computed(() => {
    return selectingFile.value.length === 1
      ? 'File'
      : selectingFile.value.length > 1
        ? 'MultiFile'
        : '';
  });
});
