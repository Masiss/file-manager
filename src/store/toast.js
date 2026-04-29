import { defineStore } from 'pinia';
import { ref } from 'vue';
import ProgressToast from '../components/Toast/ProgressToast.vue';
import SuccessToast from '../components/Toast/SuccessToast.vue';

export const useToastStore = defineStore('toast', () => {
  const toast_list = ref([]);
  const addToast = (type, data) => {
    let toast_type;
    switch (type) {
      case 'success':
        toast_type = SuccessToast;

      case 'progress':
        toast_type = ProgressToast;
    }
    toast_list.value.push({ toast_type: toast_type, data: data });
  };
  return {
    toast_list,
    addToast,
  };
});
