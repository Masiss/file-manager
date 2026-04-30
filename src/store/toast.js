import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import ProgressToast from '../components/Toast/ProgressToast.vue';
import InformToast from '../components/Toast/InformToast.vue';

export const useToastStore = defineStore('toast', () => {
  const toast_list = ref(new Map());
  const isShowAll = ref(false);
  const isPopupToast = computed(() => !isShowAll.value);
  const filtered_toast = computed(
    () =>
      new Map(
        [...toast_list.value].filter(([, val]) =>
          isShowAll.value ? val : val.visible !== false,
        ),
      ),
  );

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
  init();

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
  function hide(task_id) {
    let toast = toast_list.value.get(task_id);
    if (!toast) return;
    toast_list.value.set(task_id, { ...toast, visible: false });
  }

  return {
    toast_list,
    addToast,
    hide,
    updateProgressToast,
    isShowAll,
    filtered_toast,
    isPopupToast,
  };
});
