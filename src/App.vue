<script setup>
import { ref, shallowRef, watch, computed } from 'vue';
import { usePathStore } from './store/path.js';
import { storeToRefs } from 'pinia';
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
</script>

<template>
  <main class="container">
    <button width="100%" type="button" @click="store.$reset()">Reset</button>
    <div>
      <div>
        <button @click="store.navigate_back()"><</button>
        <button @click="store.navigate_forward()">></button>
      </div>
      <div>
        <input type="text" placeholder="Search ... " />
      </div>
    </div>
    <component :is="view" :items="items" />
  </main>
</template>

<style scoped></style>
