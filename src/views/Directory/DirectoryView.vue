<script setup>
import { usePathStore } from '../../store/path.js';
import {
  computed,
  nextTick,
  onMounted,
  ref,
  watch,
  onUnmounted,
  inject,
} from 'vue';
import { format_size, set_item_icon } from './utils.js';
import Icon from '../../components/Icon.vue';
const props = defineProps(['items']);
const lines = ref([]);
const dragRegister = inject('itemsRect', null);
const store = usePathStore();
const DISPLAY_COUNT = 10;
const current_index = ref(0);

const items_container = ref();
const container = ref();
const displaying_items = ref([]);
const sorted_items = ref([]);
const sort_items = () => {
  if (!props.items || props.items.length === 0) return [];
  sorted_items.value = [...props.items].sort((a, b) => {
    if (a.file_type < b.file_type) return -1;
    if (a.file_type > b.file_type) return 1;
    return 0;
  });
  displaying_items.value = [];
  current_index.value = 0;
  load_more();
};
const load_more = async () => {
  if (current_index.value >= sorted_items.value.length) return;
  let offset = displaying_items.value.length;
  let end_index = Math.min(
    current_index.value + DISPLAY_COUNT,
    sorted_items.value.length,
  );
  let new_items = sorted_items.value.slice(current_index.value, end_index);
  let metadata = await store.load_metadata([...new_items]);
  displaying_items.value.push(...metadata);
  current_index.value = end_index;
  nextTick(() => {
    registerLine(offset);
    check_and_fill_viewport();
  });
};
let rafId = null;
function throttle(fn) {
  if (rafId) return;
  rafId = requestAnimationFrame(() => {
    fn();
    rafId = null;
  });
}
const handle_on_scroll = (e) => {
  throttle(() => {
    let { scrollTop, clientHeight, scrollHeight } = e.target;
    let isBottom = scrollTop + clientHeight >= scrollHeight * 0.85;

    if (isBottom && current_index.value < sorted_items.value.length) {
      load_more();
    }
  });
};
const handle_resize = () => {
  throttle(check_and_fill_viewport);
};
const check_and_fill_viewport = () => {
  if (!container.value) return;
  const { scrollHeight, clientHeight } = container.value;
  if (
    scrollHeight <= clientHeight &&
    current_index.value < sorted_items.value.length
  ) {
    load_more();
  }
};
const registerLine = (offset) => {
  if (dragRegister?.register) {
    if (lines.value.length !== 0) {
      let items = lines.value.slice(offset, lines.value.length);
      dragRegister.register(...items);
    }
  }
};
watch(() => props.items, sort_items, { immediate: true, deep: true });
onMounted(() => {
  window.addEventListener('resize', handle_resize);
});

onUnmounted(() => {
  window.removeEventListener('resize', handle_resize);
  if (rafId) cancelAnimationFrame(rafId);
});
</script>
<template>
  <!-- <div ref="container" class="directory-view" @scroll="handle_on_scroll"> -->
  <table ref="container" class="table-item" @scroll="handle_on_scroll">
    <!-- class="table-item" -->
    <thead>
      <tr>
        <th>Tick</th>
        <th>icon</th>
        <th>Name</th>
        <th>Path</th>
        <th>Created at</th>
        <th>Last modified</th>
        <th>Type</th>
        <th>Size</th>
      </tr>
    </thead>
    <tbody ref="items_container">
      <tr
        v-for="(item, index) in displaying_items"
        :key="item.path"
        @click="store.access_dir(item.path, item.file_type)"
        :data-path="item.path"
        ref="lines"
      >
        <td><input type="checkbox" /></td>
        <td><Icon :icon="set_item_icon(item.file_type)" /></td>
        <td>{{ item.name }}</td>
        <td>{{ item.path }}</td>
        <td>{{ item.created_at }}</td>
        <td>{{ item.last_modified }}</td>
        <td>{{ item.type }}</td>
        <td>{{ format_size(item.size) }}</td>
      </tr>
    </tbody>
  </table>
  <!-- </div> -->
</template>
<style scoped>
.table-item {
  /* border-collapse: collapse; */
  /* display: flex; */
  /* flex-direction: column; */
}

thead {
  display: table-header-group;
  tr > th {
    font-weight: 800;
  }
}
tr {
  height: 50px;
}
th,
td {
  padding: 0 5px;
}
.directory-view {
  scroll-behavior: smooth;
}
</style>
