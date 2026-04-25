<script setup>
import { computed, onMounted, ref, shallowRef, watch } from 'vue';
import { useConfigStore } from './store/config.js';
import { emitTo, listen, once } from '@tauri-apps/api/event';
import ConflictProgress from './components/Progress/ConflictProgress.vue';
import TaskProgress from './components/Progress/TaskProgress.vue';
import { invoke } from '@tauri-apps/api/core';
const configStore = useConfigStore();
configStore.init();
const tasks = ref([]);

listen('progress-conflict', (event) => {
  console.log(event.payload);
  tasks.value.unshift({ data: event.payload, progressType: ConflictProgress });
});
listen('progress-started');
// listen('progress-started', (event) => {
//   console.log(event.payload);
//   tasks.value.unshift({ data: event.payload, progressType: TaskProgress });
// });
listen('copy-progressing', (event) => {
  let exist = tasks.value.find(
    (task) => task.data.task_id === event.payload.task_id,
  );
  console.log(exist);
  if (exist) {
    exist.data = event.payload;
  } else {
    tasks.value.unshift({ data: event.payload, progressType: TaskProgress });
  }
});
onMounted(() => {
  emitTo('main', 'progresswindow-ready');
});
watch(
  () => tasks.value,
  () => {
    console.log(tasks.value);
  },
);
const handleCancel = async (task_id) => {
  await invoke('cancel', { taskId: task_id });
  let index = tasks.value.findIndex((task) => task.data.task_id === task_id);
  if (index) {
    tasks.value.splice(index, 1);
  }
};
</script>
<template>
  <div>
    <component
      @cancel="handleCancel"
      v-for="task in tasks"
      :is="task.progressType"
      :data="task.data"
    />
  </div>
</template>

<style scoped></style>
