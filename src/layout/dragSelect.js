import { ref, onMounted, provide, watch, nextTick } from 'vue';
import { useThrottle } from '../views/Directory/utils';
export function useDragSelect(draggable_container) {
  const is_dragging = ref(false);
  const box = ref(null);
  const box_style = ref({});
  const intersected = ref([]);
  let box_start = {};
  let box_end = { x: 0, y: 0 };
  let start_scroll_top = 0;
  const { throttle } = useThrottle();

  const handleMouseDown = (e) => {
    if (e.button !== 0) return;
    if (e.target.tagName !== 'TD') return;
    box_start = { x: e.clientX, y: e.clientY };

    box_style.value.left = box_start.x + 'px';
    box_style.value.top = box_start.y + 'px';
    start_scroll_top = draggable_container.value?.scrollTop || 0;
    window.addEventListener('mousemove', handleMouseMove, {});
    window.addEventListener('mouseup', handleMouseUp, {
      once: true,
    });
    console.log('mouse down');
  };
  const scroll_speed = 15;
  const edge_threshold = 30;
  const handleMouseMove = (e) => {
    //check dragging
    let diffX = Math.abs(e.clientX - box_start.x);
    let diffY = Math.abs(e.clientY - box_start.y);
    if (diffX <= 5 && diffY <= 5) return; // chưa drag đủ ngưỡng
    if (!is_dragging.value) {
      is_dragging.value = true;
      intersected.value.forEach((item) => item.classList.remove('selected'));
      intersected.value = [];
    }

    const current_scroll = draggable_container.value.scrollTop || 0;

    box_end = { x: e.clientX, y: e.clientY };

    const containerRect = draggable_container.value.getBoundingClientRect();

    const start_content_y = box_start.y - containerRect.top + start_scroll_top;

    const end_content_y = box_end.y - containerRect.top + current_scroll;

    const top_content = Math.min(start_content_y, end_content_y);
    const bottom_content = Math.max(start_content_y, end_content_y);
    const height_content = bottom_content - top_content;

    const top_viewport = top_content - current_scroll + containerRect.top;

    let minX = Math.min(box_start.x, box_end.x);
    let new_width = Math.abs(box_start.x - box_end.x);

    box_style.value = {
      left: minX + 'px',
      top: top_viewport + 'px',
      width: new_width + 'px',
      height: height_content + 'px',
    };

    // Auto-scroll
    const rect = draggable_container.value.getBoundingClientRect();
    const mouseY = e.clientY;

    if (mouseY > rect.bottom - edge_threshold) {
      draggable_container.value.scrollBy({
        top: scroll_speed,
        behavior: 'instant',
      });
    } else if (mouseY < rect.top + edge_threshold) {
      draggable_container.value.scrollBy({
        top: -scroll_speed,
        behavior: 'instant',
      });
    }

    intersections();
    // throttle(intersections);
  };
  const handleMouseUp = (e) => {
    window.removeEventListener('mousemove', handleMouseMove);
    if (is_dragging.value) {
      is_dragging.value = false;
      box_style.value = {};
      box_start = { x: 0, y: 0 };
      box_end = { x: 0, y: 0 };
      return;
    }
    if (intersected.value.length > 0) {
      intersected.value = [];
      return;
    }
    const el = e.target.closest('tbody tr');
    intersected.value = el ? [el] : [];
  };
  const handleClick = () => {};
  watch(
    intersected,
    (newVal, oldVal) => {
      oldVal?.forEach((el) => {
        el.classList.remove('selected');
        const input = el.querySelector('input');
        if (input) input.checked = false;
      });
      newVal?.forEach((el) => {
        el.classList.add('selected');
        const input = el.querySelector('input');
        if (input) input.checked = true;
      });
    },
    { deep: false },
  );
  const intersections = () => {
    let boxRect = box.value.getBoundingClientRect();
    let newIntersections = [];
    draggable_container.value.querySelectorAll('[data-path]').forEach((el) => {
      const rect = el.getBoundingClientRect();
      const isIntersected =
        boxRect.left < rect.right &&
        boxRect.left + boxRect.width > rect.left &&
        boxRect.top < rect.bottom &&
        boxRect.top + boxRect.height > rect.top;
      if (isIntersected) newIntersections.push(el);
    });
    intersected.value = newIntersections;
  };

  return {
    is_dragging,
    draggable_container,
    box,
    box_style,
    handleMouseDown,
    handleMouseMove,
    handleMouseUp,
    intersected,
    handleClick,
  };
}
