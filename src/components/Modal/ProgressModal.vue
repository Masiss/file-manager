<script setup>
import Icon from '../Icon.vue';
</script>
<template>
  <div style="display: flex; flex-direction: column">
    <div style="display: inline-flex">
      <progress value="50" max="100"></progress> <span>50/100</span>
      <button><Icon icon="pause" /></button>
    </div>

    <div style="justify-content: end">
      <button>Cancel</button>
    </div>
  </div>
</template>
<style scoped></style>
