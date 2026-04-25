<script setup>
import { ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { format_size } from '../../views/Directory/utils.js';
import Icon from '../Icon.vue';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { invoke } from '@tauri-apps/api/core';
const props = defineProps(['data']);
</script>
<template>
  <div class="progress-container">
    <div style="display: flex; flex-direction: column; gap: 5px">
      <div
        style="
          display: flex;
          flex-direction: row;
          gap: 1rem;
          align-items: center;
        "
      >
        <Icon icon="file" icon-size="2rem" />
        <span>{{ data.filename }}</span>
      </div>
      <span style="text-overflow: ellipsis"
        >Copying file: D:\a\b\c\c\d\tetx.tx to: C:\a\a\s\d\tetx.tx</span
      >
      <span>1 of 40</span>
    </div>
    <div class="progress-info">
      <div class="progress-bar-container">
        <progress :value="data.copied" :max="data.total"></progress>
        <div
          style="
            display: flex;
            flex-direction: row;
            justify-content: space-between;
          "
        >
          <span
            >{{ format_size(data.copied) }} /
            {{ format_size(data.total) }}</span
          >
          <span>
            {{ ((data.copied / data.total) * 100).toFixed(3) }}% /100%
          </span>
        </div>
      </div>
      <button><Icon icon="pause" /></button>
    </div>

    <div class="progress-bottom">
      <button @click.prevent="$emit('cancel', data.task_id)">Cancel</button>
    </div>
  </div>
</template>
<style scoped>
.progress-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 1rem;
  padding: 5px 10px;
  .progress-info {
    display: flex;
    flex-direction: row;
    align-items: start;
    justify-content: space-around;
    gap: 5px;
    .progress-bar-container {
      width: 100%;
      height: fit-content;
      progress {
        height: 1rem;
        width: 100%;
      }
    }
  }
  .progress-bottom {
    display: flex;
    justify-content: end;
  }
}
</style>
