export function useResizing(resizingEl) {
  let m_pos, width;
  const BORDER_SIZE = 10;
  const onMouseDown = (e) => {
    e.stopPropagation();
    if (e.target.clientWidth - e.offsetX > BORDER_SIZE) return;
    m_pos = e.x;
    width = resizingEl.value.offsetWidth;
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
  };
  const onMouseMove = (e) => {
    let diff = e.clientX - m_pos;
    resizingEl.value.style.width = Math.max(50, width + diff) + 'px';
  };
  const onMouseUp = (e) => {
    // resizingEl = null;
    window.removeEventListener('mousemove', onMouseMove);
    window.removeEventListener('mouseup', onMouseUp);
  };
  const resize2Fit = (e) => {
    e.stopPropagation();
    if (resizingEl.value.offsetWidth - e.offsetX > BORDER_SIZE) return; // chỉ trigger ở cạnh phải
    e.target.style.width = 'fit-content';
  };
  return {
    onMouseDown,
    resize2Fit,
  };
}
