import { defineStore } from 'pinia';
import { ref } from 'vue';
import ProgressToast from '../components/Toast/ProgressToast.vue';
import InformToast from '../components/Toast/InformToast.vue';

export const useToastStore = defineStore('toast', () => {
  const toast_list = ref(new Map());
  function init() {
    toast_list.value.set('nfiaofasnp', {
      type: ProgressToast,
      title: 'Copy',
      data: {
        task_id: 'fioanspoansp',
        value: 15,
        total: 100,
      },
      pauseFn: () => console.log(1),
    });
    toast_list.value.set('ioasfnpas', {
      type: InformToast,
      title: 'Succcess',
      data: {
        info: 'Rename file abc.txt to abccc.txt',
      },
    });
  }

  function addToast(type, title, data, pauseFn = null) {
    init();
    toast_list.value.set(data.task_id, {
      task_id: data.task_id,
      type: type,
      title: title,
      data: data,
      pauseFn: pauseFn,
      visible: true,
    });
    console.log(toast_list.value);
  }
  function updateProgressToast(task_id, copy_progress) {
    let toast = toast_list.value.get(task_id);
    if (!toast) return;
    toast_list.value.set(task_id, { ...toast, data: { ...copy_progress } });
  }
  function hideToast(toast) {
    let index = toast_list.value.findIndex(
      (toast_item) => toast_item === toast,
    );
    if (index !== -1) {
      toast_list.value[item].visible = false;
    }
  }

  return {
    toast_list,
    addToast,
    hideToast,
    updateProgressToast,
  };
});
