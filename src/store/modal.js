import { defineStore } from 'pinia';
import { ref, shallowRef } from 'vue';
import ConfigModal from '../components/Modal/ConfigModal.vue';
import ProgressModal from '../components/Modal/ProgressModal.vue';
export const useModalStore = defineStore('modal', () => {
  const isShowing = ref(false);
  const type = shallowRef();
  const title = ref('');
  function open(modalType) {
    isShowing.value = true;
    if (modalType === 'config') {
      title.value = modalType.toUpperCase();
      type.value = ConfigModal;
    } else if (modalType === 'progress') {
      title.value = modalType.toUpperCase();
      type.value = ProgressModal;
    }
  }
  function close() {
    isShowing.value = false;
    type.value = null;
  }
  return {
    isShowing,
    close,
    open,
    type,
    title,
  };
});
