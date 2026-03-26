import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';
import { computed, ref, shallowRef, watch } from 'vue';
import DiskView from '../views/DiskView.vue';
import DirectoryView from '../views/Directory/DirectoryView.vue';

export const usePathStore = defineStore('current-path', () => {
  const current_path = ref('');
  const view = ref('disk');
  const path_list = ref(['disk:']);
  const path_index = ref(0);
  const items = ref([]);
  function navigate_forward() {
    if (path_index.value >= path_list.value.length) {
      return;
    }
    path_index.value++;
    let [newView, ...newPath] = path_list.value[path_index.value].split(':');
    view.value = newView;
    current_path.value = newPath.join(':');
    path_list.value.push(view.value + ':' + current_path.value);
  }
  function navigate_back() {
    if (path_index.value == 0) {
      return;
    }
    path_index.value--;
    let [newView, ...newPath] = path_list.value[path_index.value].split(':');
    view.value = newView;
    current_path.value = newPath.join(':');
    path_list.value.push(view.value + ':' + current_path.value);
  }
  const getView = computed(() =>
    view.value === 'search' || view.value === 'directory'
      ? DirectoryView
      : view.value === 'disk'
        ? DiskView
        : '',
  );

  function $reset() {
    current_path.value = '';
    view.value = '';
    path_index.value = 0;
    path_list.value = [];
  }
  async function search(input) {
    view.value = 'search';
    let res = [];
    if (!input) {
      return res;
    }
    path_list.value.push('search' + ':' + input);
    res = await invoke('search', { input });
    items.value = JSON.parse(res);
  }
  function access_dir(path, type) {
    console.log('access dir run');
    if (type === 'File') {
      invoke('open_file', { path });
    } else {
      current_path.value = path;
      path_list.value.push(view.value + ':' + path);
      path_index.value += 1;
      console.log(current_path.value);
    }
    console.log('access dir end');
  }
  // async function load_file() {
  //   let items = [];
  //   if (current_path.value) {
  //     view.value = 'directory';
  //     items = await invoke('load_file', { currentPath: current_path.value });
  //   } else {
  //     view.value = 'disk';
  //     items = await invoke('load_disk');
  //   }
  //   return JSON.parse(items);
  // }
  watch(
    () => current_path.value,
    () => load_path(),
    { immediate: true },
  );
  async function load_path(path) {
    let res;
    if (current_path.value) {
      view.value = 'directory';
      res = await invoke('load_path', { currentPath: current_path.value });
    } else {
      view.value = 'disk';
      res = await invoke('load_disk');
    }
    items.value = JSON.parse(res);
  }
  async function load_metadata(path) {
    let path_array = [...path];
    let res = await invoke('load_metadata', { pathList: path_array });

    return JSON.parse(res);
  }

  return {
    items,
    access_dir,
    getView,
    search,
    load_path,
    load_metadata,
    $reset,
    current_path,
    navigate_back,
    navigate_forward,
  };
});
