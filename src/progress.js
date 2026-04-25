import { createApp } from 'vue';
import Progress from './Progress.vue';

import { createPinia } from 'pinia';

const pinia = createPinia();
const app = createApp(Progress);
app.use(pinia);
app.mount('#progress');
