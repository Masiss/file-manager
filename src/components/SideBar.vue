<script setup>
import { ref, watch } from 'vue';
import { usePathStore } from '../store/path.js';
import SidebarItem from './SidebarItem.vue';
const props = defineProps(['items']);
const store = usePathStore();

const tree = ref([]);
const getSub = async (path) => {
  if (isSubExist(path)) {
    console.log('subs already loaded for ' + path);
    return;
  }
  try {
    let subs = await store.load_path(path);
    if (Array.isArray(subs) && subs) {
      insertSubs(path, subs);
    }
  } catch (err) {
    console.error('load subs failed for path:', path, err);
  }
};
const pathToNode = new Map();
const cacheNode = (node) => {
  pathToNode.set(node.path, node);
  if (node.subs) {
    node.subs.forEach(cacheNode);
  }
};
watch(
  () => props.items,
  (newItem) => {
    //if newItem empty
    if (!newItem || newItem.length === 0) return;
    //if tree empty, then push disks
    if (tree.value.length === 0) {
      let formattedDisk = newItem.map((disk) => ({
        name: disk.mount_point.split('\\')[0].concat(' ', disk.name),
        path: disk.mount_point,
        subs: [],
      }));
      tree.value = formattedDisk;
      return;
    }

    let samplePath = newItem[0];
    console.log(samplePath);
    let path = samplePath.split('\\').slice(0, -1).join('\\').concat('\\');
    if (!isSubExist(path)) {
      insertSubs(path, newItem);
    }
    console.log(tree.value);
  },
);
watch(
  tree,
  () => {
    pathToNode.clear();
    tree.value.forEach(cacheNode);
    console.log(pathToNode);
  },
  { deep: true },
);

const isSubExist = (path) => {
  let node = pathToNode.get(path);
  return node && Array.isArray(node.subs) && node.subs.length > 0;
};

const insertSubs = (parentPath, subs) => {
  let parent = pathToNode.get(parentPath);
  if (!parent) return console.warn('Parent not found:' + parentPath);

  const newSubs = subs.map((subPath) => ({
    name: subPath.split('\\').pop(),
    path: subPath.concat('\\'),
    subs: [],
  }));
  parent.subs = newSubs;
  newSubs.forEach(cacheNode);
};
</script>
<template>
  <aside class="layout_sidebar container">
    <span>Disk</span>
    <ul class="tree">
      <li v-for="node in tree">
        <SidebarItem @get-sub="getSub" :directory="node" />
      </li>
    </ul>
  </aside>
</template>
<style>
.tree {
  --spacing: 5rem;
  --radius: 10px;
}
.tree li {
  display: block;
  position: relative;
  padding-left: 3px;
}

.tree ul {
  margin-left: 6px;
  padding-left: 0;
}
</style>
