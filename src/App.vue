<script setup>
import { ref, shallowRef, watch } from 'vue';
import { usePathStore } from './store/path.js';
import { storeToRefs } from 'pinia';

import SideBar from './components/SideBar.vue';
import NavBar from './components/NavBar.vue';
import Menu from './components/Menu.vue';
import DragableView from './views/DragableView.vue';
const store = usePathStore();
const items = ref([]);
const view = shallowRef('');
const { getView } = storeToRefs(store);
watch(
  () => store.current_path,
  async () => {
    items.value = await store.load_path();
    view.value = getView.value;
  },
  { immediate: true },
);
const loadSearchItems = (search_items) => {
  items.value = search_items;
  view.value = getView.value;
};
const menuX = ref();
const menuY = ref();
const showMenu = ref(false);
const selecting_items = ref(null);
const handleContextMenu = (e) => {
  menuX.value = e.clientX;
  menuY.value = e.clientY;
  showMenu.value = true;
  selecting_items.value = e.target.parentNode.getAttribute('data-path');
  console.log();
};
</script>
<template>
  <NavBar @search="loadSearchItems" />
  <main @contextmenu.prevent="handleContextMenu" class="layout_main">
    <SideBar :items="items"></SideBar>
    <!-- <button width="100%" type="button" @click="store.$reset()">Reset</button> -->
    <DragableView>
      <component class="container" :is="view" :items="items" />
    </DragableView>
  </main>
  <Menu
    v-if="showMenu"
    :menuX="menuX"
    :menuY="menuY"
    :selectingItems="selecting_items"
  />
</template>

<style scoped></style>
