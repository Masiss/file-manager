<script setup>
import { ref, shallowRef, watch, computed } from 'vue';
import { usePathStore } from './store/path.js';
import { storeToRefs } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import SideBar from './components/SideBar.vue';
const store = usePathStore();
const items = ref([]);
const view = shallowRef('');
const { getView } = storeToRefs(store);
watch(
  () => store.current_path,
  async () => {
    items.value = await store.load_file();
    view.value = getView.value;
  },
  { immediate: true },
);
const input = ref('');
const search = async () => {
  items.value = await store.search(input.value.value);
  view.value = getView.value;
};
</script>

<template>
  <nav class="container">
    <div>
      <button @click="store.navigate_back()"><</button>
      <button @click="store.navigate_forward()">></button>
    </div>
    <div>
      <input ref="input" type="text" placeholder="Search ... " />
      <button @click="search()" type="button">seach</button>
    </div>
  </nav>
  <main>
    <SideBar></SideBar>
    <!-- <button width="100%" type="button" @click="store.$reset()">Reset</button> -->
    <component class="container content" :is="view" :items="items" />
  </main>
</template>

<style scoped></style>
