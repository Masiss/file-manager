<script setup>
import { usePathStore } from '../store/path';
const props = defineProps(['items']);
const store = usePathStore();
console.log(typeof props.items);
</script>
<template>
  <div>
    <div v-for="disk in props.items">
      <button type="button" @click="store.access_dir(disk.mount_point)">
        <span>{{ disk.mount_point }} :: {{ disk.name }} </span>
        <div>
          <progress :value="disk.usage" :max="disk.total_space" />
          <span
            >{{ (disk.available_space / 1000000000).toFixed(3) }}GB left
          </span>
        </div>
      </button>
    </div>
  </div>
</template>
<style scoped></style>
