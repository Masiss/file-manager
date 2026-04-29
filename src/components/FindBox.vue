<script setup>
import { onMounted, toRef, watch } from 'vue';
import { useFind } from '../views/Directory/find.js';
import Icon from './Icon.vue';
const props = defineProps(['lines', 'sorted_items', 'load_more']);
const emits = defineEmits(['isFinding']);

const {
  handleKeydown,
  findInPage,
  nextMatching,
  previousMatching,
  closeContainer,
  isFinding,
} = useFind(
  toRef(props, 'lines'),
  toRef(props, 'sorted_items'),
  props.load_more,
);
watch(
  () => isFinding.value,
  () => {
    emits('isFinding', isFinding.value);
  },
);

onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
});
</script>
<template>
  <div v-if="isFinding" id="findPanel" class="page-search-container container">
    <div class="page-search-item">
      <input @input="findInPage" type="text" placeholder="Type..." />
    </div>
    <div class="page-search-item">
      <button @click="previousMatching">
        <Icon icon="arrow-left-circle" icon-size="12" />
      </button>
    </div>
    <div class="page-search-item">
      <button @click="nextMatching">
        <Icon icon="arrow-right-circle" icon-size="12" />
      </button>
    </div>

    <div class="page-search-item">
      <button @click="closeContainer">
        <Icon icon="x" icon-size="12" />
      </button>
    </div>
  </div>
</template>
