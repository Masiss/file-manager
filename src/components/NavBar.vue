<script setup>
import { usePathStore } from '../store/path.js';
import { computed, ref, toRefs, nextTick, onMounted } from 'vue';
import Icon from './Icon.vue';
import { path } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/core';
const store = usePathStore();
const emits = defineEmits(['search']);

const input = ref('');
const search = async () => {
  let items = [];
  items = await store.search(input.value.value);
  emits('search', items);
};
const bread_crumbs = computed(() => {
  let current_path = store.current_path;
  let result = current_path
    .split('\\')
    .filter(Boolean)
    .flatMap((element, index, array) => {
      return index === array.length - 1 ? [element] : [element, '>'];
    });
  return result;
});
const accesss_breadcrumb = (index) => {
  let path = [...bread_crumbs.value]
    .slice(0, index + 1)
    .join('')
    .replaceAll('>', '\\');
  path = path.split('\\').length == 1 ? (path += '\\') : path;
  store.access_dir(path, 'Directory');
};
const is_editing = ref(false);
const pathInput = ref();
const edit_path = async (e) => {
  let path = pathInput.value.value;
  console.log(path);
  let is_exists = await invoke('check_path', { path: path });
  if (is_exists) {
    store.access_dir(path);
  }
};
const on_editing = async (e) => {
  let target = e.target;
  let edit_classList = [
    'bread_crumbs_container',
    'bread_crumbs',
    'bread_crumbs_edit',
    'bread_crumbs_edit_button',
  ];
  if (
    [...target.classList].some((el) => {
      return edit_classList.includes(el);
    })
  ) {
    is_editing.value = true;
    nextTick(() => {
      pathInput.value.focus();
    });
  } else {
    is_editing.value = false;
  }
};
const bread_crumbs_component = ref();
const handle_focus_out = (e) => {
  let relatedTarget = e.relatedTarget;
  if (bread_crumbs_component.value.contains(relatedTarget)) return;
  is_editing.value = false;
};
</script>
<template>
  <nav class="layout_header container">
    <div>
      <button @click="store.navigate_back()"><</button>
      <button @click="store.navigate_forward()">></button>
    </div>
    <div class="bread_crumbs_container">
      <div
        ref="bread_crumbs_component"
        class="bread_crumbs"
        @click="on_editing"
      >
        <div
          @focusout="handle_focus_out"
          class="bread_crumbs_edit_container"
          v-if="is_editing"
        >
          <input
            ref="pathInput"
            class="bread_crumbs_edit"
            type="text"
            :value="store.current_path"
          />
          <button
            class="bread_crumbs_edit_button"
            @click="edit_path"
            v-show="is_editing"
          >
            <Icon icon="edit"></Icon>
          </button>
        </div>
        <div v-else>
          <span
            class="bread_crumbs_item fioasfosa"
            v-for="(item, index) in bread_crumbs"
            @click.stop="accesss_breadcrumb(index)"
            >{{ item }}</span
          >
        </div>
      </div>
    </div>
    <div>
      <input ref="input" type="text" placeholder="Search ... " />
      <button @click="search()" type="button">seach</button>
    </div>
  </nav>
</template>
<style scoped>
.bread_crumbs {
  display: flex;
  flex-direction: row;
  justify-content: start;
  gap: 3px;
  overflow-x: clip;
  position: relative;
}
.bread_crumbs_container {
  width: 40%;
  border: 1px solid darkgray;
  position: relative;
}
.bread_crumbs_edit_container {
  width: 100%;
}

.bread_crumbs_item {
  padding: 0 7px;
  border: 0.5px solid grey;
}
.bread_crumbs_edit {
  width: 100%;
}
.bread_crumbs_edit_button {
  position: absolute;
  right: 0px;
  top: -3px;
}
</style>
