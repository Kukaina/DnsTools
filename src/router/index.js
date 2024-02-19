import { createRouter, createWebHistory } from "vue-router";
import pollute from "@/view/pollute/index.vue";
import speed from "@/view/speed/index.vue";
import about from "@/view/about/index.vue";
import settings from "@/view/settings/index.vue";

const routes = [
  {
    path: "/",
    component: speed,
    name: "speed",
    meta: { icon: "SpeedometerSharp", label: "Dns测速" },
  },
  {
    path: "/pollute",
    component: pollute,
    name: "pollute",
    meta: { icon: "WifiSharp", label: "DNS污染检测" },
  },
  {
    path: "/about",
    component: about,
    name: "about",
    meta: { icon: "InformationCircle", label: "关于" },
  },
  {
    path: "/settings",
    component: settings,
    name: "settings",
    meta: { icon: "SettingsSharp", label: "设置" },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
