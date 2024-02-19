import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index.js";
import { createPinia } from 'pinia'
import ToastPlugin from "vue-toast-notification";
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import 'vue-toast-notification/dist/theme-sugar.css';
const app = createApp(App)
const pinia=createPinia()
pinia.use(piniaPluginPersistedstate)
app.use(router)
app.use(pinia)
app.use(ToastPlugin)
app.mount('#app')
