<script setup>
import { usePathStore } from '../store/path.js';
import { ref, watch } from 'vue';
const props = defineProps(['items']);
const store = usePathStore();
</script>
<template>
  <div>
    <div>
      <div>
        <table>
          <thead>
            <tr>
              <th>Tick</th>
              <th>Name</th>
              <th>Path</th>
              <th>Created at</th>
              <th>Last modified</th>
              <th>Type</th>
              <th>Size</th>
            </tr>
          </thead>
          <tbody>
            <tr
              v-for="(item, index) in props.items"
              :key="index"
              @click="store.access_dir(item.path, item.file_type)"
            >
              <td><input type="checkbox" /></td>
              <td>{{ item.name }}</td>
              <td>{{ item.path }}</td>
              <td>{{ item.created_at }}</td>
              <td>{{ item.last_modified }}</td>
              <td>{{ item.type }}</td>
              <td>{{ item.size }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
