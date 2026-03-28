export function useMenu(x, y) {
  const x = ref(0);
  const y = ref(0);
  const isShowMenu = ref(false);
  const selecting_items = ref(null);
  return {
    showMenu,
    handleContextMenu,
  };
}
