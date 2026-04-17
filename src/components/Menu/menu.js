import { ref, watch, defineProps, computed } from 'vue';
export function useMenu(selectedItems) {
  const generalMenu = ref([
    {
      name: 'Open',
      action: () => openItem(),
    },
    {
      name: 'Copy',
      action: () => copy(),
    },
    {
      name: 'Cut',
      action: () => cut(),
    },
    {
      name: 'Paste',
      action: () => paste(),
    },
    {
      name: 'Properties',
      action: () => property(),
    },
    {
      name: 'Delete',
      action: () => deleteItem(),
    },
    {
      name: 'Rename',
      action: () => rename(),
    },
  ]);
  const fileMenu = ref([
    {
      name: 'Open with ...',
      action: () => openWith(),
      subs: {
        name: 'a',
        action: () => openWith(),
      },
    },
    {
      name: 'Run as administrator',
      action: () => openAsAdmin(),
    },
    {
      name: 'Archive to ...',
      action: () => archive(),
    },
    {
      name: 'Extract',
      action: () => extract(),
      subs: {
        name: 'Extract here',
        action: () => extract('.'),
      },
      subs: {
        name: 'Extract to ...',
        action: () => extract(),
      },
    },
  ]);
  const mixedMenu = ref([]);
  const diskMenu = ref([]);
  const combineMenu = (menuType) => {
    return [...generalMenu.value, ...(menuType?.value ?? [])];
  };
  const menu = computed(() => {
    if (selectedItems.value.length === 1) {
      return selectedItems.value.dataset?.type === 'File'
        ? combineMenu(fileMenu)
        : combineMenu();
    } else if (selectedItems.value.length > 1) {
      return combineMenu(mixedMenu);
    } else if (selectedItems.value.dataset?.type === 'Disk') {
      return combineMenu(diskMenu);
    }
  });
  const handleMenu = () => {};
  const handleCopy = (item) => {};
  const handlePaste = (item) => {};
  const handleClick = (e, action) => {};
  return {
    menu,
  };
}
