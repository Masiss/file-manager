import { defineStore } from 'pinia';
import { ref, watchEffect } from 'vue';
import {
  exists,
  BaseDirectory,
  writeTextFile,
  readTextFile,
} from '@tauri-apps/plugin-fs';
export const useConfigStore = defineStore('config', () => {
  const config = ref(null);
  const configFile = 'config.json';
  const theme = ref(localStorage.getItem('theme') || 'mocha');
  const glass = ref(localStorage.getItem('glass') === 'true');
  watchEffect(() => {
    document.documentElement.setAttribute('data-theme', theme.value);
    document.documentElement.setAttribute('data-glass', glass.value);
    localStorage.setItem('theme', theme.value);
    localStorage.setItem('glass', glass.value);
  });
  const defaultConfig = [
    {
      id: 'appearance',
      label: 'Appearance',
      icon: 'palette',
      items: [
        {
          id: 'theme',
          label: 'Theme',
          description: 'Light or dark mode',
          type: 'select',
          value: 'dark',
          options: ['latte', 'frappe', 'mocha', 'macchiato'],
        },
        {
          id: 'font_size',
          label: 'Font size',
          type: 'number',
          value: 14,
          min: 10,
          max: 24,
        },
      ],
    },
    {
      id: 'behavior',
      label: 'Behavior',
      icon: 'settings',
      items: [
        {
          id: 'open_on_single_click',
          label: 'Open on single click',
          type: 'toggle',
          value: false,
        },
        {
          id: 'show_hidden_files',
          label: 'Show hidden files',
          type: 'toggle',
          value: false,
        },
        {
          id: 'confirm_before_delete',
          label: 'Confirm before delete',
          type: 'toggle',
          value: true,
        },
      ],
    },
    {
      id: 'performance',
      label: 'Performance',
      icon: 'speed',
      items: [
        {
          id: 'page_size',
          label: 'Items per page',
          description: 'Number of items loaded at once in directory view',
          type: 'number',
          value: 50,
          min: 20,
          max: 200,
        },
        {
          id: 'thumbnail',
          label: 'Show thumbnails',
          type: 'toggle',
          value: true,
        },
      ],
    },
  ];
  async function init() {
    const cached = localStorage.getItem('config');
    if (cached) config.value = JSON.parse(cached);
    else config.value = defaultConfig;
    try {
      let checkExist = await exists(configFile, {
        baseDir: BaseDirectory.AppData,
      });
      if (!checkExist) {
        await writeTextFile(configFile, JSON.stringify(defaultConfig), {
          baseDir: BaseDirectory.AppData,
        });
        config.value = defaultConfig;
      } else {
        const raw = await readTextFile(configFile, {
          baseDir: BaseDirectory.AppData,
        });
        config.value = JSON.parse(raw);
      }
      localStorage.setItem('config', JSON.stringify(config.value));
    } catch (e) {
      console.error('Failed to load config file:', e);
    }
  }
  async function get() {
    let configText = await readTextFile(configFile, {
      baseDir: BaseDirectory.AppData,
    });
    config.value = JSON.parse(configText);
  }
  async function save() {
    await writeTextFile(configFile, JSON.stringify(configText), {
      baseDir: BaseDirectory.AppData,
    });
    localStorage.setItem('config', JSON.stringify(config.value));
  }
  function set(categoryId, itemId, value) {
    const category = this.config.find((c) => c.id === categoryId);
    const item = category?.items.find((i) => i.id === itemId);
    if (!item) return;
    item.value = value;
    applyCSSVar(item);
    if (itemId === 'theme') theme.value = value;

    save();
  }
  const applyCSSVar = (item) => {
    if (!item.cssVar) return; // chỉ apply nếu có cssVar
    document.documentElement.style.setProperty(item.cssVar, item.value);
  };
  return { config, get, save, set, init };
});
