<script setup>
const props = defineProps(['directory']);
const emit = defineEmits(['getSub']);
const handleClickExpand = (e) => {
  e.target.parentElement.open = !e.target.parentElement.open;
  emit('getSub', props.directory.path);
};
</script>
<template>
  <details>
    <summary @click.prevent="handleClickExpand">
      {{ props.directory.name }}
    </summary>
    <ul>
      <li v-for="sub in props.directory.subs">
        <SidebarItem @get-sub="$emit('getSub', sub.path)" :directory="sub" />
      </li>
    </ul>
  </details>
</template>
