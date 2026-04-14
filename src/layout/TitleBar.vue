<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { onMounted, ref, watchEffect } from 'vue';

import Icon from '../components/Icon.vue';

const appWindow = getCurrentWindow();

const iconSize = ref('14px');
onMounted(() => {
  document
    .getElementById('titlebar-minimize')
    ?.addEventListener('click', () => appWindow.minimize());
  document
    .getElementById('titlebar-maximize')
    ?.addEventListener('click', () => appWindow.toggleMaximize());
  document
    .getElementById('titlebar-close')
    ?.addEventListener('click', () => appWindow.close());
});
</script>
<template>
  <div class="titlebar">
    <div data-tauri-drag-region class="drag-region"></div>
    <div class="controls">
      <button id="titlebar-minimize" title="minimize">
        <Icon icon="minus" :iconSize="iconSize" />
      </button>
      <button id="titlebar-maximize" title="maximize">
        <Icon icon="minimize" :iconSize="iconSize" />
      </button>
      <button id="titlebar-close" title="close">
        <Icon icon="x" :iconSize="iconSize" />
      </button>
    </div>
  </div>
</template>
<style scoped>
.titlebar {
  user-select: none;
  height: 2rem;
  display: inline-flex;
}
.titlebar > * {
  height: 100%;
}
.titlebar > .drag-region {
  flex-grow: 1;
  height: 100%;
}
.titlebar > .controls {
  margin-right: 5px;
  display: inline-flex;
  justify-content: end;
  width: 10%;
}
.titlebar button {
  appearance: none;
  padding: 1rem 2rem;
  /* margin: 0 7px; */
  border: none;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 20px;
  background-color: transparent;
}
.titlebar button:hover {
  background: #5bbec3;
}
</style>
