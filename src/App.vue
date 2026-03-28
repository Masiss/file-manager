<script setup>
import NavBar from './components/NavBar.vue';
import Menu from './components/Menu/Menu.vue';
import TheMain from './layout/TheMain.vue';
import { onMounted, onUnmounted } from 'vue';
import { useKeyboard } from './composables/keyboard.js';
const { handleKeydown } = useKeyboard();
import { ref } from 'vue';
import TitleBar from './layout/TitleBar.vue';
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
    <Menu v-if="showMenu" />
  </main>
</template>

<style scoped></style>
