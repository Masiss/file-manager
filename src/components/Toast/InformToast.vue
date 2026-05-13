<script setup>
import { onMounted } from 'vue';

const props = defineProps(['data']);
</script>
<template>
  <div class="toast-body">
    {{ props.data.info }}
  </div>
</template>
