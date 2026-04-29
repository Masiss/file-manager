<script setup>
import NavBar from './components/NavBar.vue';
import Menu from './components/Menu/Menu.vue';
import TheMain from './layout/TheMain.vue';
import Modal from './components/Modal.vue';
import { onMounted, onUnmounted, watchEffect } from 'vue';
import { useKeyboard } from './composables/keyboard.js';
import { useModalStore } from './store/modal.js';
import { useConfigStore } from './store/config.js';
const { handleKeydown } = useKeyboard();
import { ref } from 'vue';
import TitleBar from './layout/TitleBar.vue';
const modal = useModalStore();
const config = useConfigStore();
config.init();
onMounted(() => {
  //handle Alt+ left/right
  window.addEventListener('keydown', handleKeydown);
});
onUnmounted(() => {
  //remove event Alt+ left/right
  window.removeEventListener('keydown', handleKeydown);
});
</script>
<template>
  <TitleBar />
  <NavBar />
  <main class="layout_main">
    <TheMain></TheMain>
    <!-- <button width="100%" type="button" @click="store.$reset()">Reset</button> -->
    <div id="bottom-line-container" class="bottom-line-container"></div>
    <div id="toast-list-container" class="toast-list-container"></div>
  </main>
  <Teleport to="body">
    <Modal v-show="modal.isShowing" />
  </Teleport>
</template>

<style scoped></style>
