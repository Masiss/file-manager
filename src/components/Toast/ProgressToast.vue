<script setup>
import { listen } from '@tauri-apps/api/event';
import Icon from '../Icon.vue';
import { useToastStore } from '../../store/toast.js';
import { onMounted } from 'vue';
const toastStore = useToastStore();

const props = defineProps(['data', 'pauseFn']);
onMounted(() => {
  listen('task-progressing', (event) => {
    let task_progress = event.payload;
    toastStore.updateProgressToast(task_progress.task_id, task_progress);
    console.log(event.payload);
  });
});
</script>
<template>
  <div class="toast-body">
    <div class="toast-info">
      <span>coping file apdasnfas to amasfasf </span>
    </div>
    <div class="toast-progress">
      <progress :value="props.data.value" :max="props.data.total"></progress>
      <button class="toast_pause-button" @click="() => props.data.pauseFn?.()">
        <Icon icon="pause" icon-size="0.8rem" />
      </button>
    </div>
  </div>
  <!-- <div class="toast-container container"> -->
  <!--   <div class="toast-top"> -->
  <!--     <span class="toast-title">title</span> -->
  <!--     <button class="toast_close-button"> -->
  <!--       <Icon icon="chevron-down" icon-size="1rem" /> -->
  <!--     </button> -->
  <!--   </div> -->
  <!--   <div class="toast-body"> -->
  <!--     <div class="toast-info"> -->
  <!--       <span>coping file apdasnfas to amasfasf </span> -->
  <!--     </div> -->
  <!--     <div class="toast-progress"> -->
  <!--       <progress value="50" max="100"></progress> -->
  <!--       <button class="toast_pause-button"> -->
  <!--         <Icon icon="pause" icon-size="0.8rem" /> -->
  <!--       </button> -->
  <!--     </div> -->
  <!--   </div> -->
  <!-- </div> -->
</template>
