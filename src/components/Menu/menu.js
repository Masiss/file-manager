import { ref, watch, defineProps, computed, useId } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Window } from '@tauri-apps/api/window';
import { getCurrentWebview, Webview } from '@tauri-apps/api/webview';
import { useModalStore } from '../../store/modal';
import { usePathStore } from '../../store/path';
import { emitTo, listen, once } from '@tauri-apps/api/event';
export function useMenu(selectingItems) {
  const modalStore = useModalStore();
  const pathStore = usePathStore();
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
  const emptyMenu = ref([
    {
      name: 'Paste',
      action: () => paste(),
    },
  ]);
  const combineMenu = (menuType) => {
    return [...generalMenu.value, ...(menuType?.value ?? [])];
  };
  const menu = computed(() => {
    if (selectingItems.value.length === 0) {
      return emptyMenu.value;
    } else if (selectingItems.value.length === 1) {
      return selectingItems.value.dataset?.type === 'File'
        ? combineMenu(fileMenu)
        : combineMenu();
    } else if (selectingItems.value.length > 1) {
      return combineMenu(mixedMenu);
    } else if (selectingItems.value.dataset?.type === 'Disk') {
      return combineMenu(diskMenu);
    }
  });
  const selectedItems = ref([]);
  const handleMenu = () => {};
  const copy = () => {
    selectedItems.value = [
      ...selectingItems.value.map((item) => item.dataset.path),
    ];
    console.log(selectedItems.value);
  };
  const createProgressWindow = async (fn) => {
    let existed = await WebviewWindow.getByLabel('progress');
    let win;
    if (!existed) {
      win = new WebviewWindow('progress', {
        label: 'progress',
        url: '../../../progress.html',
        width: 400,
        height: 250,
      });
    } else win = existed;
    await listen('progresswindow-ready');
    await fn();
    return win;
  };
  const paste = async () => {
    if (selectedItems.value.length === 0) return;
    let is_exist = await invoke('check_exist', {
      sourceList: selectedItems.value,
      destDir: pathStore.current_path,
    });
    if (is_exist.conflict) {
      let fn = async () =>
        await emitTo('progress', 'progress-conflict', {
          fileList: is_exist.conflict.file_list,
          destDir: pathStore.current_path,
        });
      await createProgressWindow(fn);
    } else if (is_exist === 'ok') {
      let taskId = crypto.randomUUID();
      let fn = async () => {
        emitTo('progress', 'progress-started');
        await invoke('copy', {
          sourceList: selectedItems.value,
          destDir: pathStore.current_path,
          taskId: taskId,
        });
      };
      await createProgressWindow(fn);
    }
  };
  const handleClick = (action) => {
    action();
  };
  return {
    menu,
    handleClick,
  };
}
