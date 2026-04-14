<script setup>
import { onMounted, ref, useTemplateRef, watch } from 'vue';
import SidebarItem from './SidebarItem.vue';
const props = defineProps(['items']);
import { useTreeView } from './treeview.js';
import { useResizing } from '../composables/resize.js';
import { invoke } from '@tauri-apps/api/core';
import { useModalStore } from '../store/modal.js';
import Icon from './Icon.vue';
const modal = useModalStore();
const quickAccess = ref();
const getQuickAccess = async () => {
  let res = await invoke('get_quick_access');
  quickAccess.value = res.map((path) => ({
    path: path,
    name: path.split('\\').pop(),
  }));
};

const sidebarRef = useTemplateRef('sidebar');
const { onMouseDown, resize2Fit } = useResizing(sidebarRef);
const { getSub, tree } = useTreeView();
onMounted(() => {
  getQuickAccess();
});
</script>
<template>
  <aside
    ref="sidebar"
    @mousedown="onMouseDown"
    @dblclick="resize2Fit"
    class="layout_sidebar container"
  >
    <div class="top-sidebar container">
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
          v-for="item in quickAccess"
          @click="getSub(item.path)"
          :key="item.path"
        >
          {{ item.name }}
        </li>
      </ul>
    </div>
    <div class="bottom-sidebar container">
      <button @click.prevent="modal.open('config')">
        <Icon icon="settings" icon-size="16" />
      </button>
    </div>
  </aside>
</template>
<style scoped>
hr {
  width: 80%;
  margin: 10px auto;
}
aside {
  overflow: auto;
  display: flex;
  flex-direction: column;
  width: auto;
  height: 100%;
  border-collapse: collapse;
  ::after {
    content: '';
    position: absolute;
    right: 0;
    top: 0;
    width: 10px;
    height: 100%;
    cursor: ew-resize;
  }
  .top-sidebar {
    flex-grow: 1;
    overflow: auto;
  }

  .bottom-sidebar {
    height: 10vh;
    width: 100%;

    button {
      border: 0;
    }
  }
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
