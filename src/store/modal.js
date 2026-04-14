import { defineStore } from 'pinia';
import { ref, shallowRef } from 'vue';
import ConfigModal from '../components/modal/ConfigModal.vue';
export const useModalStore = defineStore('modal', () => {
  const isShowing = ref(false);
  const type = shallowRef();
  const title = ref('');
  function open(modalType) {
    isShowing.value = true;
    if (modalType === 'config') {
      title.value = modalType.toUpperCase();
      type.value = ConfigModal;
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
