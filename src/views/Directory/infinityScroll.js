import { usePathStore } from '../../store/path.js';
import { computed, nextTick, onMounted, ref, watch, onUnmounted } from 'vue';
import { useThrottle } from './utils.js';
export function useInfinityScroll(items, table, scrollInfo) {
  const store = usePathStore();
  const DISPLAY_COUNT = 10;
  const current_index = ref(0);

  const { rafId, throttle } = useThrottle();
  const displaying_items = ref([]);
  const sorted_items = ref([]);
  const is_loading = ref(false);
  const sort_items = () => {
    if (!items.value || items.value.length === 0) return [];
    sorted_items.value = [...items.value].sort((a, b) => {
      if (a.file_type < b.file_type) return -1;
      if (a.file_type > b.file_type) return 1;
      return 0;
    });
    displaying_items.value = [];
    current_index.value = 0;
    load_more();
  };
  const load_more = async () => {
    console.log('load more');
    if (is_loading.value) return;
    if (current_index.value >= sorted_items.value.length) return;
    is_loading.value = true;
    let end_index = Math.min(
      current_index.value + DISPLAY_COUNT,
      sorted_items.value.length,
    );
    let new_items = sorted_items.value.slice(current_index.value, end_index);
    let metadata = await store.load_metadata([...new_items]);
    displaying_items.value.push(...metadata);
    current_index.value = end_index;
    nextTick(() => {
      is_loading.value = false;
      check_and_fill_viewport();
    });
  };
  const handleWindowResize = () => {
    console.log('handle resize');
    throttle(check_and_fill_viewport);
  };
  function check_and_fill_viewport() {
    console.log('fill view ');
    if (!scrollInfo.value) return;
    const { scrollTop, clientHeight } = scrollInfo.value;
    const { scrollHeight: tableScrollHeight } = table.value;

    if (clientHeight === 0 || tableScrollHeight === 0) return;
    let isBottom = scrollTop + clientHeight >= tableScrollHeight * 0.75;
    if (isBottom && current_index.value < sorted_items.value.length) {
      load_more();
    }
  }
  watch(
    () => scrollInfo.value,
    () => {
      throttle(check_and_fill_viewport);
    },
  );
  watch(
    items,
    () => {
      sort_items();
    },
    { immediate: true },
  );
  onMounted(() => {
    window.addEventListener('resize', handleWindowResize);
  });
  onUnmounted(() => {
    window.removeEventListener('resize', handleWindowResize);
    if (rafId) cancelAnimationFrame(rafId);
  });
  const isProgressing = () => {
    return current_index.value < sorted_items.value.length;
  };
  return {
    displaying_items,
    sort_items,
    sorted_items,
    handleWindowResize,
    isProgressing,
    current_index,
    items,
  };
}
