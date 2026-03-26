import { computed, ref, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { usePathStore } from '../store/path.js';
export function useBreadCrumb(bread_crumbs_component, pathInput) {
  const store = usePathStore();
  const bread_crumbs = computed(() => {
    let current_path = store.current_path;
    let result = current_path
      .split('\\')
      .filter(Boolean)
      .flatMap((element, index, array) => {
        return index === array.length - 1 ? [element] : [element, '>'];
      });
    return result;
  });
  const accesssBreadcrumb = (index) => {
    let path = [...bread_crumbs.value]
      .slice(0, index + 1)
      .join('')
      .replaceAll('>', '\\');
    path = path.split('\\').length == 1 ? (path += '\\') : path;
    store.access_dir(path, 'Directory');
  };
  const isEditing = ref(false);
  const editPath = async (e) => {
    let path = pathInput.value.value;
    let is_exists = await invoke('check_path', { path: path });
    if (is_exists) {
      store.access_dir(path);
    }
    isEditing.value = false;
  };
  const onEditing = async (e) => {
    let target = e.target;
    let edit_classList = [
      'bread_crumbs_container',
      'bread_crumbs',
      'bread_crumbs_edit',
      'bread_crumbs_edit_button',
    ];
    if (
      [...target.classList].some((el) => {
        return edit_classList.includes(el);
      })
    ) {
      isEditing.value = true;
      nextTick(() => {
        pathInput.value.focus();
      });
    } else {
      isEditing.value = false;
    }
  };
  const handleFocusOut = (e) => {
    console.log(e);
    let relatedTarget = e.relatedTarget;
    console.log(e.target);
    if (bread_crumbs_component.value.contains(relatedTarget)) return;
    isEditing.value = false;
  };
  const cancelEditing = () => {
    pathInput.value.blur();
    isEditing.value = false;
  };
  return {
    bread_crumbs,
    isEditing,
    handleFocusOut,
    onEditing,
    editPath,
    accesssBreadcrumb,
    cancelEditing,
  };
}
