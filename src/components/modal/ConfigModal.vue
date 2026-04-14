<script setup>
import { computed, onMounted, ref, watch } from 'vue';
import { useConfigStore } from '../../store/config.js';

const configStore = useConfigStore();
const currentLabel = ref(null);
const currentItems = computed(() => {
  return configStore.config.find((item) => item.label === currentLabel.value)
    ?.items;
});

const changeLabel = (label) => {
  currentLabel.value = label;
};
let labels = ref();
onMounted(() => {
  labels.value = configStore.config?.map((config) => config.label);
  currentLabel.value = labels.value[0];
});
</script>
<template>
  <div class="config-container">
    <div class="config-left">
      <ul>
        <li v-for="label in labels">
          <button @click="changeLabel(label)">
            {{ label }}
          </button>
        </li>
      </ul>
    </div>
    <hr style="border-right: 1px solid" />
    <div class="config-main">
      <div class="config-top">
        <div v-for="item in currentItems">
          <span class="config-label"> {{ item.label }} </span>
          <input
            class="config-control"
            v-if="item.type === 'number'"
            type="number"
            :value="item.value"
          />
          <select class="config-control" v-if="item.type === 'select'">
            <option v-for="option in item.options" :key="option">
              {{ option }}
            </option>
          </select>
          <label class="config-control switch" v-if="item.type === 'toggle'">
            <input type="checkbox" :checked="item.value" />
            <span class="slider round"></span>
          </label>
        </div>
      </div>
      <div class="config-bottom">
        <div>
          <button @click="configStore.save">Save</button>
        </div>
        <div>
          <button>Default</button>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
.switch {
  position: relative;
  display: inline-block;
  width: 1.5rem;
  height: 1.5rem;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

.slider:before {
  position: absolute;
  content: '';
  height: 1rem;
  width: 1rem;
  left: 4px;
  bottom: 4px;
  background-color: white;
  -webkit-transition: 0.4s;
  transition: 0.4s;
}

input:checked + .slider {
  background-color: #2196f3;
}

input:focus + .slider {
  box-shadow: 0 0 1px #2196f3;
}

input:checked + .slider:before {
  -webkit-transform: translateX(26px);
  -ms-transform: translateX(26px);
  transform: translateX(26px);
}

/* Rounded sliders */
.slider.round {
  border-radius: 3rem;
}

.slider.round:before {
  border-radius: 50%;
}
.config-container {
  display: flex;
  flex-direction: row;
  width: fit-content;
  height: auto;
  overflow-x: clip;
  padding: 5px 10px;
  gap: 0.5rem;
  ul > li {
    margin: 0.5rem 0;
  }
  .config-left {
    display: flex;
    flex-direction: column;
    gap: 10px;
    button {
      width: 100%;
      text-align: left;
    }
  }
  .config-main {
    display: flex;
    flex-direction: column;
    padding: 0 0.5rem;
    .config-top {
      flex-grow: 1;
      > div {
        display: inline-flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;
        gap: 1rem;
        .config-control {
          text-align: end;
          field-sizing: content;
          min-width: 4rem;
        }
        .config-label {
          white-space: nowrap;
          flex-shrink: 0;
        }
      }
    }
    .config-bottom {
      width: 100%;
      display: inline-flex;
      justify-content: end;
      gap: 0.5rem;
    }
  }
}
</style>
