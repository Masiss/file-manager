import { ref, watch } from 'vue';
export function useFind(lines, itemList, load_more) {
  const foundItems = ref([]);
  const currentItemIndex = ref(null);
  const lastKey = ref(null);
  const isFinding = ref(false);
  const findInPage = (e) => {
    isFinding.value = true;
    let input = e.target.value;
    foundItems.value = itemList.value.filter((item) =>
      item.split('\\').pop().includes(input),
    );
    if (foundItems.value.length === 0) return;
    currentItemIndex.value = 0;
  };
  const nextMatching = () => {
    currentItemIndex.value =
      currentItemIndex.value === foundItems.value.length - 1
        ? 0
        : currentItemIndex.value + 1;
  };
  const previousMatching = () => {
    currentItemIndex.value =
      currentItemIndex.value === 0
        ? foundItems.value.length - 1
        : currentItemIndex.value - 1;
  };
  const closeContainer = () => {
    currentItemIndex.value = null;
    foundItems.value = [];
    lastKey.value = null;
    isFinding.value = false;
    lines.value?.forEach((line) => line.classList.remove('selected'));
  };
  watch(
    () => currentItemIndex.value,
    (val) => {
      if (val === null) return;
      let path = foundItems.value[val];
      if (!jumpToEl(path)) scrollToItem(path);
    },
  );
  const jumpToEl = (path) => {
    const el = lines.value.find((line) => line.dataset.path === path);
    if (!el) return false;
    el.scrollIntoView({ behavior: 'smooth', block: 'center' });
    selectNewItem(el);
    return true;
  };
  const scrollToItem = (path) => {
    //scroll to find item
    const interval = setInterval(() => {
      load_more();
      //find item
      if (jumpToEl(path) || lines.value.length === itemList.value.length) {
        clearInterval(interval);
      }
    }, 100);
  };
  const selectNewItem = (el) => {
    lines.value.forEach((line) => line.classList.remove('selected'));
    el.classList.add('selected');
  };
  const handleKeydown = (e) => {
    if (!/^[a-zA-Z]$/.test(e.key)) return;

    if (e.ctrlKey && e.key === 'f') {
      e.preventDefault();
      isFinding.value = !isFinding.value;
    }
    if (isFinding.value) return;
    //if key === lastKey -> next item
    if (e.key === lastKey.value) {
      currentItemIndex.value =
        currentItemIndex.value === foundItems.value.length - 1
          ? 0
          : (currentItemIndex.value += 1);
    }
    //else find item
    else {
      let key = e.key;
      lastKey.value = key;
      foundItems.value = itemList.value.filter((item) =>
        item.split('\\').pop().toLowerCase().startsWith(key),
      );
      currentItemIndex.value = 0;

      if (foundItems.value.length === 0) return;
    }
  };
  return {
    handleKeydown,
    findInPage,
    nextMatching,
    previousMatching,
    closeContainer,
    isFinding,
  };
}
