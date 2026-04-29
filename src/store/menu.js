import { ref, watch, computed, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { usePathStore } from './path';
import { emitTo, listen, once } from '@tauri-apps/api/event';
import { useModalStore } from './modal';
import { defineStore } from 'pinia';
export const useMenuStore = defineStore('menu', () => {
  const selectingItems = ref([]);
  const isMenuShow = ref(false);
  const x = ref(null);
  const y = ref(null);
  const clipboard = ref([]);

  const modalStore = useModalStore();
  const pathStore = usePathStore();
  const menuItems = ref([
    {
      name: 'Open',
      action: () => openItem(),
      visible: () => isSelecting(),
    },
    {
      name: 'Open with ...',
      action: () => openWith(),
      visible: () => isSelecting(),
      subs: [
        {
          name: 'a',
          action: () => openWith(),
        },
      ],
    },
    {
      name: 'Run as administrator',
      action: () => openAsAdmin(),
      visible: () => isSelecting(),
    },
    {
      name: 'Copy',
      action: () => copy(),
      visible: () => isSelecting(),
    },
    {
      name: 'Cut',
      action: () => cut(),
      visible: () => isSelecting(),
    },
    {
      name: 'Paste',
      action: () => paste(),
      visible: () => clipboard.value.length > 0,
    },
    {
      name: 'Properties',
      action: () => property(),
      visible: () => isSelecting(),
    },
    {
      name: 'Delete',
      action: () => deleteItem(),
      visible: () => isSelecting(),
    },
    {
      name: 'Rename',
      action: () => rename(),
      visible: () => selectingItems.value.length === 1,
    },
    {
      name: 'Archive to ...',
      action: () => archive(),
    },
    {
      name: 'Extract',
      action: () => extract(),
      visible: () => isCompressed() && isSelecting(),
      subs: [
        {
          name: 'Extract here',
          action: () => extract('.'),
        },
        {
          name: 'Extract to ...',
          action: () => extract(),
        },
      ],
    },
  ]);
  const isSelecting = () => selectingItems.value.length > 0;
  const isCompressed = () => {
    let compress_type = ['zip', 'tar', 'gz', '7z', 'rar', 'bz2', 'xz'];
    console.log(selectingItems.value);
    return selectingItems.value.every((item) => {
      let ext = item.cells[1].innerText.split('.').pop();
      return compress_type.includes(ext);
    });
  };
  const menu = computed(() => {
    console.log(selectingItems.value);
    return menuItems.value.filter((item) => !item.visible || item.visible());
  });
  const copy = () => {
    clipboard.value = [
      ...selectingItems.value.map((item) => item.dataset.path),
    ];
  };
  const cut = () => {
    copy();
  };
  const renamingPath = ref(null);
  const rename = async () => {
    let item = selectingItems.value[0];
    let path = item.dataset.path;
    let name_cell = item.querySelector('span.name-span');
    let old_name = name_cell.innerText;
    renamingPath.value = path;
    const preventBubble = (e) => e.stopPropagation();
    const confirmRename = async () => {
      // await invoke('rename', {
      //   sourceStr: renamingPath.value,
      //   newName: document.querySelector('td[contenteditable="true"]').innerText,
      // });
    };
    const keydownConfirm = (e) => {
      // e.preventDefault();
      e.stopPropagation();
      if (e.key === 'Enter') {
        name_cell.blur();
      } else if (e.key === 'Escape') {
        cancelRename();
        cleanUp();
      }
    };

    const handleRenameBlur = (e) => {
      if (e.target.innerText !== old_name.trim()) {
        modalStore.open(
          'confirm',
          {
            info: `Do you want to rename from ${old_name} to ${e.target.innerText} ?`,
            yes: () => {
              confirmRename();
              modalStore.close(true);
            },
            no: () => {
              cancelRename();
              modalStore.close();
            },
          },
          () => cancelRename(),
        );
      } else {
        cancelRename();
      }
      renamingPath.value = null;
    };
    const cleanUp = () => {
      name_cell.removeEventListener('mousedown', preventBubble);
      name_cell.removeEventListener('dblclick', preventBubble);
      name_cell.removeEventListener('keydown', keydownConfirm);
      name_cell.removeEventListener('blur', handleRenameBlur);
    };
    name_cell.addEventListener('mousedown', preventBubble);
    name_cell.addEventListener('dblclick', preventBubble);
    name_cell.addEventListener('keydown', keydownConfirm);
    name_cell.addEventListener('blur', handleRenameBlur);
    await nextTick(() => {
      name_cell.focus();
    });
    const cancelRename = () => {
      console.log('cancel rename run');
      name_cell.innerText = old_name;
      renamingPath.value = null;
      cleanUp();
    };
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
    await once('progresswindow-ready');
    await fn();
    return win;
  };
  const paste = async () => {
    if (clipboard.value.length === 0) return;
    let is_exist = await invoke('check_exist', {
      sourceList: clipboard.value,
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
          sourceList: clipboard.value,
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
  function handleContextMenu(e) {
    x.value = e.clientX;
    y.value = e.clientY;
    isMenuShow.value = true;
    nextTick(() => {
      window.addEventListener('click', closeContextMenu);
    });
  }
  function closeContextMenu(e) {
    if (!e.target.closest('div#menu') || !e.target.closest('li.menu-item'))
      isMenuShow.value = false;
    else nextTick(() => (isMenuShow.value = false));
  }
  return {
    menu,
    handleClick,
    renamingPath,
    isMenuShow,
    x,
    y,
    selectingItems,
    handleContextMenu,
  };
});
