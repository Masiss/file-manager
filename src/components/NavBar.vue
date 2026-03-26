<script setup>
import { usePathStore } from '../store/path.js';
import { onMounted, onUnmounted, useTemplateRef, computed } from 'vue';
import Icon from './Icon.vue';
import { useBreadCrumb } from './breadcrumb.js';
const store = usePathStore();

const input = useTemplateRef('searchInput');
const search = async () => {
  await store.search(input.value.value);
};
const pathInput = useTemplateRef('pathInput');
const bread_crumbs_component = useTemplateRef('bread_crumbs_component');
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
  <nav class="layout_header container">
    <div>
      <button @click.left="store.navigate_back()">
        <Icon icon="chevron-left" />
      </button>
      <button @click.left="store.navigate_forward()">
        <Icon icon="chevron-right" />
      </button>
    </div>
    <div
      class="bread_crumbs_container"
      v-if="isShowBreadcrumb"
      @focusout="handleFocusOut"
    >
      <div ref="bread_crumbs_component" class="bread_crumbs" @click="onEditing">
        <div class="bread_crumbs_edit_container" v-if="isEditing">
          <input
            ref="pathInput"
            class="bread_crumbs_edit"
            type="text"
            @keydown.enter="editPath"
            @keydown.esc="cancelEditing"
            :value="store.current_path"
          />
          <button
            class="bread_crumbs_edit_button"
            @click="editPath"
            v-show="isEditing"
          >
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
    <div>
      <input
        ref="searchInput"
        @keydown.enter="search"
        @keydown.esc="cancelEditing"
        type="text"
        placeholder="Search ... "
      />
      <button @click="search" type="button">seach</button>
    </div>
  </nav>
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
  width: 100%;
}

.bread_crumbs_item {
  padding: 0 7px;
  border: 0.5px solid grey;
}
.bread_crumbs_edit {
  width: 100%;
}
.bread_crumbs_edit_button {
  position: absolute;
  right: 0px;
  top: -3px;
}
</style>
