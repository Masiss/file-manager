<script setup>
const props = defineProps(['data']);
import Icon from '../Icon.vue';
</script>
<template>
  <div class="toast-container container">
    <div class="toast-top">
      <span class="toast-title">title</span>
      <button class="toast_close-button">
        <Icon icon="chevron-down" icon-size="1rem" />
      </button>
    </div>
    <div class="toast-body">
      <div class="toast-info">
        <span>coping file apdasnfas to amasfasf </span>
      </div>
      <div class="toast-progress">
        <progress value="50" max="100"></progress>
        <button class="toast_pause-button">
          <Icon icon="pause" icon-size="0.8rem" />
        </button>
      </div>
    </div>
  </div>
</template>
