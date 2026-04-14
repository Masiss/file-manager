import { ref, watch } from 'vue';
export function useFind(lines, container, itemList) {
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
    currentItemIndex.value = 0;
    let path = foundItems.value[currentItemIndex];
    if (!jumpToEl(path)) scrollToItem(path);
  };
  const nextMatching = () => {
    currentItemIndex.value =
      currentItemIndex.value === foundItems.value.length - 1
        ? 0
        : (currentItemIndex.value += 1);
  };
  const closeContainer = () => {
    currentItemIndex.value = null;
    foundItems.value = [];
    lastKey.value = null;
    isFinding.value = false;
  };
  watch(
    () => currentItemIndex.value,
    () => {
      let path = foundItems.value[currentItemIndex.value];
      if (!jumpToEl(path)) scrollToItem(path);
    },
  );
  const previousMatching = () => {
    currentItemIndex.value =
      currentItemIndex.value === 0
        ? foundItems.value.length - 1
        : (currentItemIndex.value -= 1);
  };
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
      container.scrollBy({ top: 200, behavior: 'instant' });
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
    console.log(e.key);
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
    //get path at index in founded list
    // const path = foundItems.value[currentItemIndex.value];
    // //is currently exist as element
    // if (!jumpToEl(path)) scrollToItem(path);
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
