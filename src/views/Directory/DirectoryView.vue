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
import Icon from '../../components/Icon.vue';
import { useInfinityScroll } from './infinityScroll.js';
import { useResizing } from '../../composables/resize.js';
import BottomLine from '../../components/BottomLine/BottomLine.vue';
import { invoke } from '@tauri-apps/api/core';
import { useMenuStore } from '../../store/menu.js';
import FindBox from '../../components/FindBox/FindBox.vue';
const props = defineProps(['items', 'isDragging', 'scrollInfo', 'intersected']);
const pathStore = usePathStore();
const menuStore = useMenuStore();
const table = useTemplateRef('table');
const emits = defineEmits(['line-click']);
const items = ref([]);
watch(
  () => props.items,
  () => (items.value = props.items),
  { immediate: true },
);
const {
  displaying_items,
  isProgressing,
  sorted_items,
  current_index,
  load_more,
} = useInfinityScroll(items, table, toRef(props, 'scrollInfo'));
const ths = ref();
const lines = useTemplateRef('lines');
const isAsc = ref(false);
const sortingCol = ref(null);
const sortColumn = async (colName) => {
  sortingCol.value = colName;
  isAsc.value = !isAsc.value;
  console.log(colName);
  let res = await invoke('sort_column', {
    columnName: colName,
    pathList: sorted_items.value,
    ascending: isAsc.value,
  });
  items.value = res;
};
const cols = ref([
  {
    name: 'name',
    label: 'Name',
  },
  {
    name: 'created_at',
    label: 'Created at',
  },
  {
    name: 'last_modified',
    label: 'Last modified',
  },
  {
    name: 'path',
    label: 'Path',
  },
  {
    name: 'size',
    label: 'Size',
  },
]);
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
const isRenaming = (path) => {
  return path === menuStore.renamingPath;
};
const isFinding = ref(null);
</script>
<template>
  <div class="directory-view">
    <Teleport to="#progressbar_container">
      <progress
        id="loading_progressbar"
        :value="current_index"
        :max="sorted_items.length"
        v-show="isProgressing()"
      >
        {{ current_index }}
      </progress>
    </Teleport>
    <FindBox
      :lines="lines"
      :sorted_items="sorted_items"
      :load_more="load_more"
      @isFinding="(state) => (isFinding = state)"
    />
    <table ref="table" class="table-item">
      <thead>
        <tr>
          <th v-if="showCheckbox" style="width: 3vw"></th>
          <th v-for="col in cols" @click.prevent="sortColumn(col.name)">
            <div>
              <span> {{ col.label }}</span>
              <span v-if="sortingCol === col.name" class="direction-container">
                <Icon v-if="isAsc" icon="chevron-up" icon-size="12px" />
                <Icon v-else icon="chevron-down" icon-size="12px" />
              </span>
            </div>
          </th>
        </tr>
      </thead>
      <tbody ref="items_container">
        <tr
          v-for="item in displaying_items"
          :key="item.path"
          @dblclick="pathStore.access_dir(item.path, item.file_type)"
          :data-path="item.path"
          :data-type="item.type"
          ref="lines"
        >
          <td v-if="showCheckbox" style="width: 3vw">
            <input :checked="isSelected(item.path)" type="checkbox" />
          </td>
          <td class="name-cell">
            <Icon
              :icon="set_item_icon(item.file_type)"
              v-if="menuStore.renamingPath !== item.path"
            />
            <span class="name-span" :contenteditable="isRenaming(item.path)">
              {{ item.name }}
            </span>
          </td>
          <td>{{ item.created_at }}</td>
          <td>{{ item.last_modified }}</td>
          <td>{{ item.path }}</td>
          <td>{{ format_size(item.size) }}</td>
        </tr>
      </tbody>
    </table>
    <Teleport to="#bottom-line-container">
      <BottomLine
        :itemsLength="sorted_items.length"
        :loadedLength="displaying_items.length"
        :isDragging="props.isDragging"
        :selectedLength="intersected.length"
        :isFinding="isFinding"
      />
    </Teleport>
  </div>
</template>
<style scoped>
.name-cell {
}
.name-span {
  white-space: nowrap;
  margin-left: 10px;
}

#loading_progressbar {
  width: 100%;
  height: 5px;
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
  > div {
    width: 100%;
    display: inline-flex;
    justify-content: space-around;
    align-items: center;
    :first-child {
      flex-grow: 1;
    }
    .direction-container {
      display: flex;
      flex-direction: column;
      justify-content: space-around;
    }
  }
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
  scrollbar-width: none;
}
</style>
