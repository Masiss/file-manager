<script setup>
import { usePathStore } from '../../store/path.js';
import { ref, inject, useTemplateRef, toRef, onMounted } from 'vue';
import { format_size, set_item_icon } from './utils.js';
import Icon from '../../components/Icon.vue';
import { useInfinityScroll } from './infinityScroll.js';
import { useResizing } from '../../composables/resize.js';
const props = defineProps(['items', 'isDragging', 'scrollInfo']);
const store = usePathStore();
const table = useTemplateRef('table');
const { displaying_items, isProgressing, sorted_items, current_index } =
  useInfinityScroll(toRef(props, 'items'), table, toRef(props, 'scrollInfo'));
const ths = ref();
const lines = useTemplateRef('lines');
onMounted(() => {
  ths.value = table.value.querySelectorAll('th');
  let cols = Array.from(ths.value).map((th) => ({
    width: th.getBoundingClientRect().width,
  }));
  ths.value.forEach((th, i) => {
    th = ref(th);
    const { onMouseDown } = useResizing(th);
    //set default col width
    th.value.style.width = cols[i].width + 'px';
    //set fit col width
    th.value.addEventListener('dblclick', (e) => {
      //get max width
      let maxWidth = lines.value.reduce(
        (max, currentValue) => Math.max(max, currentValue.cells[i].scrollWidth),
        0,
      );
      //padding
      maxWidth += (5 / 100) * maxWidth;
      if (maxWidth === e.target.clientWidth) return;
      e.target.style.width = maxWidth + 'px';
    });
    th.value.addEventListener('mousedown', onMouseDown);
  });
});
</script>
<template>
  <div class="directory-view">
    <Teleport to="#draggable_container">
      <progress
        id="loading_progressbar"
        :value="current_index"
        :max="sorted_items.length"
        v-show="isProgressing()"
      >
        {{ current_index }}
      </progress>
    </Teleport>
    <table ref="table" class="table-item">
      <thead>
        <tr>
          <th v-show="isDragging"></th>
          <th>Name</th>
          <th>Created at</th>
          <th>Last modified</th>
          <th>Path</th>
          <th>Size</th>
        </tr>
      </thead>
      <tbody ref="items_container">
        <tr
          v-for="item in displaying_items"
          :key="item.path"
          @click.stop="store.access_dir(item.path, item.file_type)"
          :data-path="item.path"
          :data-type="item.type"
          ref="lines"
        >
          <td v-show="isDragging"><input type="checkbox" /></td>
          <td class="name-cell">
            <!-- <span> -->
            <Icon :icon="set_item_icon(item.file_type)" />
            {{ item.name }}
            <!-- </span> -->
          </td>
          <td>{{ item.created_at }}</td>
          <td>{{ item.last_modified }}</td>
          <td>{{ item.path }}</td>
          <td>{{ format_size(item.size) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
<style scoped>
.name-cell {
  gap: 5px;
}
#loading_progressbar {
  width: 100vw;
  height: 5px;
  margin: 0 auto;
  position: sticky;
  top: 0px;
}
.table-item {
  border-collapse: collapse;
  border: none;
  overflow: scroll;
  min-width: 100%;
  width: fit-content;
  table-layout: fixed;
}
th {
  font-weight: 800;
  position: relative;
  border-right: 1px solid grey;
}
th::after {
  content: '';
  position: absolute;
  right: 0;
  top: 0;
  height: 100%;
  width: 3px;
  cursor: ew-resize;
}

tr {
  height: 50px;
  align-items: center;
}

th,
td {
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}
td {
  padding: 0 5px;
}
.directory-view {
  overflow: auto;
  width: 100%;
  scroll-behavior: smooth;
}
</style>
