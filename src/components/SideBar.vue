<script setup>
import { ref, useTemplateRef, watch } from 'vue';
import SidebarItem from './SidebarItem.vue';
const props = defineProps(['items']);
import { useTreeView } from './treeview.js';
import { useResizing } from '../composables/resize.js';

const sidebarRef = useTemplateRef('sidebar');
const { onMouseDown, resize2Fit } = useResizing(sidebarRef);
const { getSub, tree } = useTreeView();
const quickAccesses = ref([
  {
    name: 'Downloads',
    path: '%USER%\\Downloads\\',
  },
  {
    name: 'app',
    path: 'D:\\app\\',
  },
]);
</script>
<template>
  <aside
    ref="sidebar"
    @mousedown="onMouseDown"
    @dblclick="resize2Fit"
    class="layout_sidebar container"
  >
    <span>Disk</span>
    <ul class="tree">
      <li v-for="node in tree">
        <SidebarItem @get-sub="getSub" :directory="node" />
      </li>
    </ul>
    <hr />
    <span> Quick accesses </span>
    <ul>
      <li
        v-for="item in quickAccesses"
        @click="getSub(item.path)"
        :key="item.path"
      >
        {{ item.name }}
      </li>
    </ul>
  </aside>
</template>
<style scoped>
hr {
  width: 80%;
  margin: 10px auto;
}
aside {
  overflow: auto;
}
aside::after {
  content: '';
  position: absolute;
  right: 0;
  top: 0;
  width: 10px;
  height: 100%;
  cursor: ew-resize;
}
.tree {
  --spacing: 5rem;
  --radius: 10px;
}
.tree li {
  display: block;
  position: relative;
  padding-left: 3px;
  width: fit-content;
}

.tree ul {
  margin-left: 6px;
  padding-left: 0;
}
</style>
