export function useKeyboard() {
  const handleKeydown = (e) => {
    if (e.altKey && e.key === 'ArrowLeft') {
      e.preventDefault();
      store.navigate_back();
    }
    if (e.altKey && e.key === 'ArrowRight') {
      e.preventDefault();
      store.navigate_forward();
    }
  };
  return {
    handleKeydown,
  };
}
