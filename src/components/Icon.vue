<script setup>
import { watch, onMounted, ref, nextTick, computed } from 'vue';
import feather from 'feather-icons';

const props = defineProps(['icon', 'iconSize']);
let width, height;
width = height = props.iconSize ?? '18px';
const iconSvg = computed(() => feather.icons[props.icon].toSvg(iconObj));

let iconObj = { width: width, height: height, 'stroke-width': '3' };
onMounted(() => {
  feather.replace(iconObj);
});
</script>
<template>
  <div class="icon-container">
    <span v-html="iconSvg"></span>
  </div>
</template>
<style>
.icon-container {
  display: inline-block;
  width: fit-content;
  height: fit-content;
  padding: 0;
  margin: 0;
}
</style>
