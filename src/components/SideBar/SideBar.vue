<script setup>
import { onMounted, ref, useTemplateRef, watch } from 'vue';
import SidebarItem from './SidebarItem.vue';
const props = defineProps(['items']);
import { useTreeView } from '../treeview.js';
import { useResizing } from '../../composables/resize.js';
import { invoke } from '@tauri-apps/api/core';
import { useModalStore } from '../../store/modal.js';
import { useConfigStore } from '../../store/config.js';
import { usePathStore } from '../../store/path.js';

import Icon from '../Icon.vue';
const modalStore = useModalStore();
const configStore = useConfigStore();
const pathStore = usePathStore();
const sidebarRef = useTemplateRef('sidebar');

onMounted(() => configStore.getQuickAccess());
const { onMouseDown, resize2Fit } = useResizing(sidebarRef);
const { getSub, tree } = useTreeView();
</script>
<template>
  <aside
    @contextmenu.prevent="handleContextMenu"
    ref="sidebar"
    @mousedown="onMouseDown"
    @dblclick="resize2Fit"
    class="layout_sidebar container"
  >
    <div class="top-sidebar container">
      <span>Disk</span>
      <ul class="tree">
        <li class="node" v-for="node in tree">
          <SidebarItem @get-sub="getSub" :directory="node" />
        </li>
      </ul>
      <hr />
      <span class="button" @click.prevent="pathStore.trash_dir">Trash Bin</span>
      <hr />
      <span> Quick access </span>
      <ul>
        <li
          class="quick-access-item"
          v-for="item in configStore.quickAccess"
          @click="getSub(item.path)"
          :key="item.path"
          :data-path="item.path"
        >
          {{ item.name }}
        </li>
      </ul>
    </div>
    <div class="bottom-sidebar container">
      <button @click.prevent="modalStore.open('config')">
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
