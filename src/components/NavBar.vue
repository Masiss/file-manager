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
    <div class="search-container">
      <input
        ref="searchInput"
        @keydown.enter="search"
        @keydown.esc="cancelEditing"
        type="text"
        placeholder="Search ... "
      />
      <button @click="search" type="button">
        <Icon icon="search" icon-size="16" />
      </button>
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
.search-container {
  position: relative;
  input {
    width: 20vw;
    max-width: 200px;
    min-width: 100px;
  }
  button {
    padding: 0.25rem 0.5rem;
  }
}
</style>
