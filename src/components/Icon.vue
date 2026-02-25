<script setup>
import { onMounted } from 'vue';
import feather from 'feather-icons';

const props = defineProps(['icon']);
onMounted(() => {
  feather.replace();
});
</script>
<template>
  <i :data-feather="props.icon"></i>
</template>
