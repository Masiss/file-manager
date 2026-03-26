<script setup>
import { useTemplateRef, computed, ref } from 'vue';
import Icon from './Icon.vue';
const props = defineProps(['directory']);
const emit = defineEmits(['getSub']);
const details = useTemplateRef('details');
const isOpen = ref(false);
const icon = computed(() => {
  return isOpen.value ? 'arrow-down' : 'arrow-right';
});
const handleClickExpand = (e) => {
  details.value.open = !details.value.open;
  emit('getSub', props.directory.path);
  console.log(props.directory.path);
  isOpen.value = !isOpen.value;
};
</script>
<template>
  <details ref="details">
    <summary @click.prevent="handleClickExpand" ref="summary">
      <Icon :icon="icon" />
      <span>
        {{ props.directory.name }}
      </span>
    </summary>
    <ul>
      <li v-for="sub in props.directory.subs">
        <SidebarItem @get-sub="$emit('getSub', $event)" :directory="sub" />
      </li>
    </ul>
  </details>
</template>
<style>
.icon-container {
  width: fit-content;
  height: fit-content;
}
details > * {
  padding-left: 10px;
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}
summary:first-of-type {
  padding-left: 0;
}

summary:hover,
ul > li:hover {
  opacity: 0.8;
  /* background-color: white; */
  border: 0.5px solid grey;
}
summary {
  display: inline-flex;
  width: fit-content;
  align-content: center;
}
summary > span {
  width: 100px;
}
</style>
