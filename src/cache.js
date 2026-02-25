class Cache {
  items = new Map();
  async set(dir, paths) {
    if (!dir || paths.length === 0) return [];
    if (items.get(dir)) return [];
    items.set(dir, [...paths]);
  }
  async get(dir) {
    if (!dir) return [];
    return items.get(dir);
  }
  async clear() {
    items.clear();
  }
}
