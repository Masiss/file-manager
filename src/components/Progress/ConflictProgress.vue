<script setup>
import Icon from '../Icon.vue';
const props = defineProps(['data']);
</script>
<template>
  <div>
    <div v-for="file in props.data.fileList" class="container">
      <div style="display: flex; flex-direction: row">
        <div style="display: flex; flex-direction: column">
          <Icon icon="file" />
          <span> {{ props.data.fileList }} </span>
        </div>
        <div>
          <span
            >Existed file named {{ props.data.fileList }} at
            {{ props.data.dest_dir }}</span
          >
        </div>
      </div>
      <div style="display: flex; flex-direction: row; justify-content: end">
        <button>Overwrite</button>
        <button>Rename</button>
        <button>Cancel</button>
      </div>
    </div>
  </div>
</template>
<style scoped></style>
