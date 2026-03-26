import { ref } from 'vue';
export function set_item_icon(file_type) {
  if (file_type === 'Directory') {
    return 'folder';
  }
  return 'file';
}

export function format_size(bytes) {
  if (!bytes && bytes !== 0) return '-';
  if (bytes === 0) return '0 B';

  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}
export function useThrottle() {
  let rafId = null;
  function throttle(fn) {
    if (rafId) return;
    rafId = requestAnimationFrame(() => {
      fn();
      rafId = null;
    });
  }
  return {
    rafId,
    throttle,
  };
}
