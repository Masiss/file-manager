import { ref, watch } from 'vue';
import { usePathStore } from '../store/path.js';
export function useTreeView() {
  const store = usePathStore();

  const tree = ref([]);
  const getSub = async (path) => {
    console.log(path);
    if (path === store.current_path) return;
    try {
      store.access_dir(path);
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
    () => store.items,
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
      if (typeof newItem[0] === 'string') {
        let samplePath = newItem[0];
        let path = samplePath?.split('\\').slice(0, -1).join('\\').concat('\\');
        if (!isSubExist(path)) {
          insertSubs(path, newItem);
        }
      }
    },
  );
  watch(
    tree,
    () => {
      pathToNode.clear();
      tree.value.forEach(cacheNode);
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
  return {
    getSub,
    tree,
  };
}
