<script setup>
import { onUnmounted, watch } from 'vue';

const props = defineProps(['data']);
</script>
<template>
  <div class="confirm-modal-container">
    <div class="confirm-modal-top">
      <span>
        {{ props.data.info }}
      </span>
    </div>
    <div class="confirm-modal-bottom">
      <button @click="props.data.no">No</button>
      <button @click="props.data.yes">Yes</button>
    </div>
  </div>
</template>
<style scoped>
.confirm-modal-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px 20px;
}
.confirm-modal-top {
  justify-content: center;
}
.confirm-modal-bottom {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}
</style>
