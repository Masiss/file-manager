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
  window.addEventListener('keydown', handleKeydown);
});
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>
<template>
  <TitleBar />
  <NavBar />
  <main @contextmenu.prevent="handleContextMenu" class="layout_main">
    <TheMain></TheMain>
    <!-- <button width="100%" type="button" @click="store.$reset()">Reset</button> -->
  </main>
  <!-- <Menu -->
  <!--   v-if="showMenu" -->
  <!--   :menuX="menuX" -->
  <!--   :menuY="menuY" -->
  <!--   :selectingItems="selecting_items" -->
  <!-- /> -->
</template>

<style scoped></style>
