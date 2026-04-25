<template>
  <div
    class="toast-container container"
    style="position: absolute; right: 1rem; bottom: 1.5rem; padding: 5px 3px"
  >
    <div style="display: flex; flex-direction: column">
      <div style="display: inline-flex">
        <span style="flex-grow: 1">title</span>
        <button style="background: none; border: none">
          <Icon icon="chevron-down" icon-size="12px" />
        </button>
      </div>
      <div
        style="
          display: inline-flex;
          justify-content: space-around;
          align-items: center;
        "
      >
        <progress value="50" max="100" style="height: 0.75rem"></progress>
        <button style="background: none; border: none; align-content: center">
          <Icon icon="pause" icon-size="8px" />
        </button>
      </div>
    </div>
  </div>
</template>
