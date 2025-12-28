import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref, shallowRef } from 'vue';
import DiskView from '../views/DiskView.vue';
import DirectoryView from '../views/DirectoryView.vue';

export const usePathStore = defineStore('current-path', () => {
  const current_path = ref('');
  const view = shallowRef('');
  const path_list = ref([]);
  const path_index = ref(0);
  function navigate_forward() {
    console.log('navigate forward');
    console.log(path_list.value);
    console.log(path_index.value);
    if (path_index.value >= path_list.value.length) {
      return;
    }
    path_index.value++;
    current_path.value = path_list.value[path_index.value - 1];
  }
  function navigate_back() {
    console.log('navigate back');
    console.log(path_list.value);
    console.log(path_index.value);
    if (path_index.value == 0) {
      return;
    }
    path_index.value--;
    current_path.value = path_list.value[path_index.value - 1];
    console.log(path_list.value[path_index.value]);
  }
  const getView = computed(() => view.value);
  function $reset() {
    current_path.value = '';
    view.value = '';
    path_index.value = 0;
    path_list.value = [];
  }
  function access_dir(path) {
    current_path.value = path;
    path_list.value.push(path);
    path_index.value += 1;
  }
  async function load_file() {
    let items = [];
    if (current_path.value) {
      view.value = DirectoryView;
      items = await invoke('load_file', { currentPath: current_path.value });
    } else {
      view.value = DiskView;
      items = await invoke('load_disk');
    }
    return JSON.parse(items);
  }

  return {
    access_dir,
    getView,
    load_file,
    $reset,
    current_path,
    navigate_back,
    navigate_forward,
  };
});
