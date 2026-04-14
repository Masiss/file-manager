<script setup>
import { usePathStore } from '../../store/path.js';
import {
  ref,
  inject,
  useTemplateRef,
  toRef,
  onMounted,
  computed,
  watch,
} from 'vue';
import { format_size, set_item_icon } from './utils.js';
import { useFind } from './find.js';
import Icon from '../../components/Icon.vue';
import { useInfinityScroll } from './infinityScroll.js';
import { useResizing } from '../../composables/resize.js';
const props = defineProps(['items', 'isDragging', 'scrollInfo', 'intersected']);
const store = usePathStore();
const table = useTemplateRef('table');
const { displaying_items, isProgressing, sorted_items, current_index } =
  useInfinityScroll(toRef(props, 'items'), table, toRef(props, 'scrollInfo'));
const ths = ref();
const lines = useTemplateRef('lines');
const draggable_container = document.querySelector('#draggable_container');
const {
  handleKeydown,
  findInPage,
  nextMatching,
  previousMatching,
  closeContainer,
  isFinding,
} = useFind(lines, draggable_container, sorted_items);
onMounted(() => {
  document.addEventListener('keydown', handleKeydown);
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
      if (maxWidth === e.target.clientWidth) return;
      //padding
      maxWidth += (5 / 100) * maxWidth;
      e.target.style.width = maxWidth + 'px';
    });
    th.value.addEventListener('mousedown', onMouseDown);
  });
});
const showCheckbox = computed(() => {
  return props.isDragging || props.intersected?.length > 0;
});
const isSelected = (path) => {
  return props.intersected?.some((el) => el.dataset.path === path) ?? false;
};
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
    <div v-if="isFinding" class="page-search-container container">
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
    <table ref="table" class="table-item">
      <thead>
        <tr>
          <th v-if="showCheckbox" style="width: 3vw"></th>
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
          @dblclick="store.access_dir(item.path, item.file_type)"
          :data-path="item.path"
          :data-type="item.type"
          ref="lines"
        >
          <td v-if="showCheckbox" style="width: 3vw">
            <input :checked="isSelected(item.path)" type="checkbox" />
          </td>
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
  width: 100%;
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
  width: 100%;
  table-layout: fixed;
}
th {
  font-weight: 800;
  position: relative;
  border-right: 1px solid grey;
  min-width: 80px;
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
.page-search-container {
  display: inline-flex;
  gap: 0.5rem;
  /* max-width: 300px; */
  position: fixed;
  z-index: 100;
  /* top: 0; */
  right: 1rem;
  padding: 2px 5px;
  background: var(--bg-panel);
  input {
    width: 10rem;
  }
  .page-search-item {
    border: 0;
    :hover {
      border: 1px solid black;
    }
  }
}
.directory-view {
  overflow-x: auto;
  width: 100%;
  min-width: 100%;
  position: relative;
  scroll-behavior: smooth;
}
</style>
