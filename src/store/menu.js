import {
  ref,
  watch,
  computed,
  nextTick,
  useTemplateRef,
  triggerRef,
  toRef,
} from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { usePathStore } from './path';
import { emitTo, listen, once } from '@tauri-apps/api/event';
import { useModalStore } from './modal';
import { defineStore, storeToRefs } from 'pinia';
import { useToastStore } from './toast.js';
import { useConfigStore } from './config.js';
export const useMenuStore = defineStore('menu', () => {
  const selectingItems = ref([]);
  const isMenuShow = ref(false);
  const x = ref(null);
  const y = ref(null);
  const clipboard = ref({ item_list: [], type: null });
  const menuRef = ref(null);
  const closestRow = ref(null);

  const configStore = useConfigStore();
  const toastStore = useToastStore();
  const modalStore = useModalStore();
  const pathStore = usePathStore();
  let context = ref('main');
  const sidebarItem = ref([]);
  const confirmFn = ref(null);
  const cancelFn = ref(null);

  const menuItems = ref([
    {
      name: 'Open',
      action: () => openItem(),
      visible: () => isSelecting(),
    },
    {
      name: 'Add to quick access',
      action: () => addQuickAccess(),
      visible: () => isSelecting(),
    },
    {
      name: 'New file',
      action: () => createFile(),
    },
    {
      name: 'New directory',
      action: () => addNewDirectory(),
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
      visible: () => clipboard.value.item_list.length > 0,
    },
    {
      name: 'Move to bin',
      action: () => moveToBin(),
      visible: () => isSelecting(),
    },
    {
      name: 'Delete',
      action: () => deleteItem(),
      visible: () => isSelecting(),
    },
    {
      name: 'Properties',
      action: () => property(),
      visible: () => isSelecting() || isSideBarItem(),
    },

    {
      name: 'Rename',
      action: () => rename(),
      visible: () => selectingItems.value.length === 1,
    },
    {
      name: 'Archive',
      visible: () => isSelecting(),
      subs: [
        {
          name: 'to 7z',
          action: () => archive('7z'),
        },
        {
          name: 'to zip',
          action: () => archive('zip'),
        },
        {
          name: 'to tar',
          action: () => archive('tar'),
        },
      ],
    },
    {
      name: 'Extract',
      visible: () => isCompressed() && isSelecting(),
      subs: [
        {
          name: 'Extract here',
          action: () => extract(),
        },
      ],
    },
  ]);
  const sidebarMenu = ref([
    {
      name: 'Properties',
      action: () => property(),
      visible: () => isSelecting() || isSideBarItem(),
    },
    {
      name: 'Remove quick access',
      action: () => removeQuickAccess(),
      visible: () => isQuickAccess(),
    },
  ]);
  const haveNewFile = ref(null);
  const createFile = async () => {
    let file_name = 'New file.txt';
    let new_path = pathStore.current_path + file_name;
    let tbody = document.querySelector(`tr`);
    console.log(tbody);
    let clone = closestRow.value.cloneNode(true);
    let name_span = clone.querySelector('span.name-span');
    const cancelFn = () => {
      name_span.innerText = file_name;
      name_span.contentEditable = false;
    };
    const confirmFn = async () => {
      try {
        let new_name = name_span.innerText;
        await invoke('create_file', {
          directory: pathStore.current_path,
          fileName: new_name,
        });
      } catch (e) {
        toastStore.addToast('inform', 'Error', { info: e });
      }
      pathStore.reload();
      clone.remove();
    };
    name_span.addEventListener('keydown', (e) => {
      e.stopPropagation();
      if (e.key === 'Enter') {
        e.preventDefault();
        e.target.blur();
      } else if (e.key === 'Esc') {
        cancelFn();
      }
    });
    name_span.addEventListener('blur', (e) => {
      confirmFn();
    });
    name_span.innerText = file_name;
    closestRow.value.after(clone);
    name_span.contentEditable = true;
    name_span.focus();
  };
  const deleteItem = async (permanent = false) => {
    let taskId = crypto.randomUUID();

    await createProgressWindow();
    await invoke('delete_file', {
      taskId: taskId,
      pathList: selectingItems.value.map((item) => item.dataset.path),
      permanent: permanent,
    });
  };
  const isQuickAccess = () => {
    return sidebarItem.value[0]?.closest('li.quick-access-item');
  };
  const isSideBarItem = () => {
    return sidebarItem.value[0]?.closest(
      'ul.tree, li.node, li.quick-access-item',
    );
  };
  const isSelecting = () => selectingItems.value.length > 0;
  const isCompressed = () => {
    let compress_type = ['zip', 'tar', 'gz', '7z', 'rar', 'bz2', 'xz'];
    return selectingItems.value.every((item) => {
      let ext = item.querySelector('span.name-span').innerText.split('.').pop();
      return compress_type.includes(ext);
    });
  };
  const openItem = () => {
    let item_type = selectingItems.value[0].dataset.fileType;
    let path = selectingItems.value[0].dataset.path;
    pathStore.access_dir(path, item_type);
  };
  const menu = computed(() => {
    return context.value === 'main'
      ? menuItems.value.filter((item) => !item.visible || item.visible())
      : context.value === 'sidebar'
        ? sidebarMenu.value.filter((item) => !item.visible || item.visible())
        : [];
  });
  const addQuickAccess = () => {
    configStore.addQuickAccess(selectingItems.value[0].dataset.path);
  };
  const removeQuickAccess = () => {
    let path = sidebarItem.value[0].closest('li.quick-access-item')?.dataset
      .path;
    invoke('remove_quick_access', {
      path: path,
    });
  };
  const archive = async (format) => {
    let taskId = crypto.randomUUID();
    await createProgressWindow();

    await invoke('archive', {
      taskInfo: {
        src_list: selectingItems.value.map((item) => item.dataset.path),
        dest_dir: pathStore.current_path,
        task_id: taskId,
      },
      format: format.trim(),
    });
    await listen('message', (e) => console.log(e));
  };
  const extract = async (dest, password) => {
    let taskId = crypto.randomUUID();
    await createProgressWindow();
    await invoke('extract', {
      taskInfo: {
        src_list: selectingItems.value.map((item) => item.dataset.path),
        dest_dir: pathStore.current_path,
        task_id: taskId,
      },
    });
  };
  const copy = () => {
    clipboard.value = {
      item_list: [...selectingItems.value.map((item) => item.dataset.path)],
      type: 'copy',
    };
  };
  const cut = () => {
    clipboard.value = {
      item_list: [...selectingItems.value.map((item) => item.dataset.path)],
      type: 'cut',
    };
  };
  const renamingPath = ref(null);
  const rename = async () => {
    let item = selectingItems.value[0];
    let name_cell = item.querySelector('span.name-span');
    let old_name = name_cell.innerText;
    renamingPath.value = selectingItems.value[0].dataset.path;

    confirmFn.value = async (newName) => {
      if (name_cell.innerText !== old_name.trim()) {
        modalStore.open(
          'confirm',
          {
            info: `Do you want to rename from ${old_name} to ${name_cell.innerText} ?`,
            yes: () => {
              confirmRename();
              modalStore.close(true);
            },
            no: () => {
              cancelFn.value();
              modalStore.close();
            },
          },
          () => cancelFn.value(),
        );
      } else {
        cancelFn.value();
      }
      const confirmRename = async () => {
        await invoke('rename', {
          sourceStr: renamingPath.value,
          newName: newName,
        });
        pathStore.reload();
        toastStore.addToast('inform', 'Success', {
          info: `Renamed ${old_name} to ${newName}`,
        });
      };
    };

    cancelFn.value = () => {
      name_cell.innerText = old_name;
      renamingPath.value = null;
    };
  };
  //   const rename = async () => {
  //   let item = selectingItems.value[0];
  //   let path = item.dataset.path;
  //   let name_cell = item.querySelector('span.name-span');
  //   let old_name = name_cell.innerText;
  //   renamingPath.value = path;
  //   const preventBubble = (e) => e.stopPropagation();
  //   const confirmRename = async () => {
  //     let newName = name_cell.innerText;
  //     await invoke('rename', {
  //       sourceStr: renamingPath.value,
  //       newName: newName,
  //     });
  //     toastStore.addToast('inform', 'Success', {
  //       info: `Renamed ${old_name} to ${newName}`,
  //     });
  //   };
  //   const keydownConfirm = (e) => {
  //     // e.preventDefault();
  //     e.stopPropagation();
  //     if (e.key === 'Enter') {
  //       name_cell.blur();
  //     } else if (e.key === 'Escape') {
  //       cancelRename();
  //       cleanUp();
  //     }
  //   };
  //
  //   const handleRenameBlur = (e) => {
  //     if (e.target.innerText !== old_name.trim()) {
  //       modalStore.open(
  //         'confirm',
  //         {
  //           info: `Do you want to rename from ${old_name} to ${e.target.innerText} ?`,
  //           yes: () => {
  //             confirmRename();
  //             modalStore.close(true);
  //           },
  //           no: () => {
  //             cancelRename();
  //             modalStore.close();
  //           },
  //         },
  //         () => cancelRename(),
  //       );
  //     } else {
  //       cancelRename();
  //     }
  //   };
  //   const cleanUp = () => {
  //     name_cell.removeEventListener('mousedown', preventBubble);
  //     name_cell.removeEventListener('dblclick', preventBubble);
  //     name_cell.removeEventListener('keydown', keydownConfirm);
  //     name_cell.removeEventListener('blur', handleRenameBlur);
  //   };
  //   name_cell.addEventListener('mousedown', preventBubble);
  //   name_cell.addEventListener('dblclick', preventBubble);
  //   name_cell.addEventListener('keydown', keydownConfirm);
  //   name_cell.addEventListener('blur', handleRenameBlur);
  //   await nextTick(() => {
  //     name_cell.focus();
  //   });
  //   const cancelRename = () => {
  //     console.log('cancel rename run');
  //     name_cell.innerText = old_name;
  //     renamingPath.value = null;
  //     cleanUp();
  //   };
  // };
  const createProgressWindow = async () => {
    let existed = await WebviewWindow.getByLabel('progress');
    if (existed) {
      return;
    }

    const ready = once('progresswindow-ready');
    new WebviewWindow('progress', {
      label: 'progress',
      url: '../../../progress.html',
      width: 400,
      height: 250,
    });
    await ready;
    // await fn();
    return;
  };
  const paste = async () => {
    if (clipboard.value.item_list.length === 0 || !clipboard.value.type) return;
    let type = clipboard.value.type;
    let is_exist = await invoke('check_exist', {
      sourceList: clipboard.value.item_list,
      destDir: pathStore.current_path,
    });
    let taskId = crypto.randomUUID();
    if (is_exist.conflict) {
      let fn = async () =>
        await emitTo('progress', 'progress-conflict', {
          task_id: taskId,
          file_list: is_exist.conflict.file_list,
          dest_dir: pathStore.current_path,
        });
      toastStore.addToast('inform', 'Conflict', {
        task_id: taskId,
        file_list: is_exist.conflict.file_list,
        dest_dir: pathStore.current_path,
      });
      await createProgressWindow();
      await fn();
    } else if (is_exist === 'ok') {
      let command = type === 'copy' ? 'copy' : 'cut';
      let fn = async () => {
        await emitTo('progress', 'progress-started');
        await invoke(command, {
          taskInfo: {
            src_list: clipboard.value.item_list,
            dest_dir: pathStore.current_path,
            task_id: taskId,
          },
        });
      };
      toastStore.addToast('progress', type.toUpperCase(), { task_id: taskId });
      await createProgressWindow(fn);
      await fn();
    }
  };
  const handleClick = (action) => {
    if (!action) return;
    isMenuShow.value = false;
    action();
  };
  function handleContextMenu(e) {
    e.preventDefault();
    if (e.target.closest('aside.layout_sidebar')) {
      context.value = 'sidebar';
      sidebarItem.value = [e.target];
    } else {
      context.value = 'main';
    }
    closestRow.value = e.target.closest('tr');
    x.value = e.clientX;
    y.value = e.clientY;
    isMenuShow.value = true;
    nextTick(() => {
      window.addEventListener('click', closeContextMenu);
    });
  }
  async function closeContextMenu(e) {
    e.preventDefault();
    e.stopPropagation();
    if (!menuRef.value.contains(e.target)) isMenuShow.value = false;
    window.removeEventListener('click', closeContextMenu);
  }
  return {
    menu,
    menuRef,
    handleClick,
    renamingPath,
    isMenuShow,
    x,
    y,
    selectingItems,
    handleContextMenu,
    confirmFn,
    cancelFn,
    haveNewFile,
  };
});
