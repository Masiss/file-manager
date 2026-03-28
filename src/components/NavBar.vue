<script setup>
import { usePathStore } from '../store/path.js';
import { onMounted, onUnmounted, useTemplateRef, computed } from 'vue';
import Icon from './Icon.vue';
import BreadCrumb from './Breadcrumb.vue';
const store = usePathStore();

const input = useTemplateRef('searchInput');
const search = async () => {
  await store.search(input.value.value);
};
</script>
<template>
  <nav class="layout_header container">
    <div class="left_header">
      <div>
        <button @click.left="store.navigate_back()">
          <Icon icon="chevron-left" />
        </button>
        <button @click.left="store.navigate_forward()">
          <Icon icon="chevron-right" />
        </button>
      </div>
      <button @click.left="store.access_dir('')">
        <Icon icon="home" />
      </button>
    </div>
    <BreadCrumb />
    <div>
      <input
        ref="searchInput"
        @keydown.enter="search"
        @keydown.esc="cancelEditing"
        type="text"
        placeholder="Search ... "
      />
      <button @click="search" type="button">seach</button>
    </div>
  </nav>
</template>
<style scoped>
.left_header {
  display: inline-flex;
  gap: 10px;
}
.left_header > div {
  justify-content: space-around;
}
</style>
