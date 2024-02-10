import { createApp } from "vue";
import App from "./Index.vue";
import router from "./router/index.js";
const app = createApp(App)
app.use(router)
app.mount('#app')
