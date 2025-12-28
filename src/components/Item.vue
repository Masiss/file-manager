<script setup>
import { usePathStore } from "../store/path.js";
const store = usePathStore();
</script>
<template>
  <div @click="store.access_dir()">
    <slot></slot>
  </div>
</template>
