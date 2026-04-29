import { defineStore } from 'pinia';
import { ref, shallowRef } from 'vue';
import ConfigModal from '../components/Modal/ConfigModal.vue';
import ConfirmModal from '../components/Modal/ConfirmModal.vue';
export const useModalStore = defineStore('modal', () => {
  const isShowing = ref(false);
  const type = shallowRef();
  const title = ref('');
  const modalInfo = ref(null);
  const handleConfirm = ref({ yes: null, no: null });
  const onClose = ref(null);
  function open(modalType, data, closeFn = null) {
    isShowing.value = true;
    modalInfo.value = data;
    if (closeFn) onClose.value = closeFn;
    if (modalType === 'config') {
      title.value = modalType.toUpperCase();
      type.value = ConfigModal;
    } else if (modalType === 'confirm') {
      title.value = modalType.toUpperCase();
      type.value = ConfirmModal;
    }
  }
  function close(isConfirm = false) {
    isShowing.value = false;
    type.value = null;
    if (onClose.value && !isConfirm) {
      onClose.value();
      onClose.value = null;
    }
  }
  return {
    isShowing,
    close,
    open,
    type,
    title,
    modalInfo,
    handleConfirm,
  };
});
