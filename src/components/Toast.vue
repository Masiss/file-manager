<script setup>
import { onMounted, watch } from 'vue';
import { useToastStore } from '../store/toast.js';
import Icon from './Icon.vue';
import InformToast from './Toast/InformToast.vue';
import ProgressToast from './Toast/ProgressToast.vue';
const toastStore = useToastStore();
watch(
  () => toastStore.toast_list,
  () => console.log(toastStore.toast_list),
  {
    immediate: true,
  },
);
const getToastType = (type) => {
  switch (type) {
    case 'inform':
      return InformToast;

    case 'progress':
      return ProgressToast;
  }
};
</script>
<template>
  <div class="toast-list" v-if="toastStore.toast_list.size > 0">
    <div
      v-for="[index, toast] in toastStore.toast_list"
      class="toast-container container"
    >
      <div class="toast-top">
        <span class="toast-title">{{ toast.title }}</span>
        <button class="toast_close-button">
          <Icon icon="chevron-down" icon-size="1rem" />
        </button>
      </div>
      <div>
        <component
          :is="getToastType(toast.type)"
          :data="toast.data"
          :pauseFn="toast.pauseFn"
        />
      </div>
    </div>
  </div>
</template>
<style scoped></style>
