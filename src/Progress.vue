<script setup>
import { computed, onMounted, ref, shallowRef, watch } from 'vue';
import { useConfigStore } from './store/config.js';
import { emitTo, listen, once } from '@tauri-apps/api/event';
import ConflictProgress from './components/Progress/ConflictProgress.vue';
import TaskProgress from './components/Progress/TaskProgress.vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
const configStore = useConfigStore();
configStore.init();
const tasks = ref(new Map());

listen('progress-conflict', (event) => {
  console.log(event.payload);
  tasks.value.set(event.payload.task_id, {
    ...event.payload,
    progress_type: 'conflict',
  });
});
listen('progress-started');
listen('task-progressing', (event) => {
  let task = tasks.value.get(event.payload.task_id);
  if (task) {
    tasks.value.set(task.task_id, { ...task, ...event.payload });
  } else {
    tasks.value.set(event.payload.task_id, {
      ...event.payload,
      progress_type: 'task',
    });
  }
  console.log(tasks.value);
});
onMounted(() => {
  emitTo('main', 'progresswindow-ready');
});
const progressComponent = (progress_type) => {
  return progress_type === 'conflict'
    ? ConflictProgress
    : progress_type === 'task'
      ? TaskProgress
      : '';
};
const activeTask = computed(() =>
  [...tasks.value.values()].filter(
    (task) => task.done !== true || task.value < task.total,
  ),
);
watch(
  () => activeTask.value,
  () => {
    console.log(activeTask.value);
    if (activeTask.value.length === 0) getCurrentWebviewWindow().close();
  },
);
const handleCancel = async (task_id) => {
  await invoke('cancel', { taskId: task_id });
  tasks.value.delete(task_id);
};
</script>
<template>
  <div>
    <component
      @cancel="handleCancel"
      v-for="task in activeTask"
      :is="progressComponent(task.progress_type)"
      :data="task"
    />
  </div>
</template>

<style scoped></style>
