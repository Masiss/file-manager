<script setup>
import { useModalStore } from '../store/modal.js';
import Icon from '../components/Icon.vue';
const modal = useModalStore();
</script>
<template>
  <div class="modal container">
    <div class="modal-top">
      <span>{{ modal.title }}</span>
      <button @click.prevent="modal.close()">
        <Icon icon="x" icon-size="12" />
      </button>
    </div>
    <component :is="modal.type" v-if="modal.type" />
  </div>
</template>
<style scoped>
.modal {
  position: absolute;
  margin-inline: auto;
  display: flex;
  width: max-content;
  flex-direction: column;
  left: 35%;
  top: 25%;
  .modal-top {
    display: flex;
    align-items: center;
    flex-direction: row;
    height: 30px;
    width: 100%;
    span {
      flex-grow: 1;
      padding-left: 10px;
    }
  }
}
</style>
