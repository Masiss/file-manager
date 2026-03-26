export function useMenu() {
  const menuX = ref();
  const menuY = ref();
  const showMenu = ref(false);
  const selecting_items = ref(null);
  const handleContextMenu = (e) => {
    menuX.value = e.clientX;
    menuY.value = e.clientY;
    showMenu.value = true;
    selecting_items.value = e.target.parentNode.getAttribute('data-path');
    console.log();
  };
}
