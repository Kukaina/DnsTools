import { createApp } from "vue";
import App from "./App.vue";
import router from "./router/index.js";
import { createPinia } from "pinia";
import ToastPlugin from "vue-toast-notification";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import "vue-toast-notification/dist/theme-sugar.css";
const app = createApp(App);
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
app.use(router);
app.use(pinia);
app.use(ToastPlugin);
app.mount("#app");
import { useSettingsStore } from "./store/settings.js";
//背景图片设置
const settings = useSettingsStore();
document.body.style.backgroundImage = `url(${settings.backgroundimg})`;
document.body.style.backgroundRepeat = "no-repeat";
document.body.style.backgroundSize = "cover"; // 确保图片覆盖整个元素
document.body.style.backgroundPosition = "center"; // 图片居中显示
