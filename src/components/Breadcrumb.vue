<script setup>
import { usePathStore } from '../store/path.js';
import { useBreadCrumb } from './breadcrumb.js';
import { onMounted, onUnmounted, useTemplateRef, computed } from 'vue';
import Icon from './Icon.vue';
const pathInput = useTemplateRef('pathInput');
const bread_crumbs_component = useTemplateRef('bread_crumbs_component');
const store = usePathStore();
const {
  bread_crumbs,
  accesssBreadcrumb,
  onEditing,
  isEditing,
  handleFocusOut,
  editPath,
  cancelEditing,
} = useBreadCrumb(bread_crumbs_component, pathInput);
const isShowBreadcrumb = computed(() => {
  return bread_crumbs.value.length > 0;
});
</script>
<template>
  <div
    class="bread_crumbs_container"
    v-if="isShowBreadcrumb"
    @focusout="handleFocusOut"
  >
    <div ref="bread_crumbs_component" class="bread_crumbs" @click="onEditing">
      <div class="bread_crumbs_edit_container" v-if="isEditing">
        <input
          ref="pathInput"
          class="bread_crumbs_input"
          type="text"
          @keydown.enter="editPath"
          @keydown.esc="cancelEditing"
          :value="store.current_path"
        />
        <button class="bread_crumbs_edit_button" @click="editPath">
          <!-- v-show="true" -->
          <Icon icon="edit"></Icon>
        </button>
      </div>
      <div v-else>
        <span
          class="bread_crumbs_item"
          v-for="(item, index) in bread_crumbs"
          :key="index"
          @click.stop="accesssBreadcrumb(index)"
          >{{ item }}</span
        >
      </div>
    </div>
  </div>
</template>
<style scoped>
.bread_crumbs {
  display: flex;
  flex-direction: row;
  justify-content: start;
  gap: 3px;
  overflow-x: clip;
  position: relative;
}
.bread_crumbs_container {
  width: 40%;
  border: 1px solid darkgray;
  position: relative;
}
.bread_crumbs_edit_container {
  display: inline-flex;
  width: 100%;
  align-items: center;
}

.bread_crumbs_item {
  padding: 0 0.5rem;
  border: 0.5px solid grey;
}
.bread_crumbs_input {
  flex-grow: 1;
}
.bread_crumbs_edit_button {
  width: 2rem;
  align-items: center;
}
</style>
