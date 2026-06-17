<script setup>
import { computed } from 'vue';
import Icon from '../Icon.vue';
const props = defineProps(['data']);
const hasTotal = computed(() => Number(props.data.total) > 0);
const value = computed(() => Number(props.data.value) || 0);
const total = computed(() => Number(props.data.total) || 0);
const percent = computed(() =>
  hasTotal.value ? Math.min((value.value / total.value) * 100, 100).toFixed(1) : '0.0',
);
const title = computed(() => props.data.title || 'Processing');
const statusText = computed(() => {
  if (!hasTotal.value) return `${value.value} processed`;
  return `${value.value} / ${total.value}`;
});
</script>
<template>
  <div class="progress-container">
    <div class="progress-header">
      <div class="progress-title">
        <Icon icon="file" icon-size="2rem" />
        <span>{{ title }}</span>
      </div>
      <span class="progress-status">{{ statusText }}</span>
    </div>
    <div class="progress-info">
      <div class="progress-bar-container">
        <progress
          v-if="hasTotal"
          :value="value"
          :max="total"
        ></progress>
        <progress v-else></progress>
        <div class="progress-meta">
          <span>{{ statusText }}</span>
          <span>
            {{ hasTotal ? `${percent}%` : 'Working' }}
          </span>
        </div>
      </div>
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
  .progress-header {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  .progress-title {
    display: flex;
    flex-direction: row;
    gap: 1rem;
    align-items: center;
  }
  .progress-status {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
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
    .progress-meta {
      display: flex;
      flex-direction: row;
      justify-content: space-between;
      gap: 1rem;
    }
  }
  .progress-bottom {
    display: flex;
    justify-content: end;
  }
}
</style>
