<script setup>
import { useToastStore } from '../store/toast.js';
import Icon from './Icon.vue';
import InformToast from './Toast/InformToast.vue';
import ProgressToast from './Toast/ProgressToast.vue';
const toastStore = useToastStore();
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
  <div class="toast-list">
    <div
      v-for="[index, toast] in toastStore.filtered_toast"
      class="toast-container container"
    >
      <div class="toast-top">
        <span class="toast-title">{{ toast.title }}</span>
        <button
          v-if="!toastStore.isShowAll"
          class="toast_close-button"
          @click.prevent="toastStore.hide(index)"
        >
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
