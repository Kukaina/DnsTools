import { defineStore } from "pinia";
import { ref } from "vue";

export const useSettingsStore = defineStore("settings", {
  state: () => ({
    filePath: ref("./DnsList.json"),
    firstwarn: false,
    zoom: 1,
    theme: null,
    filternum: 0,
    backgroundimg: "",
    DarkthemeOverrides: {
      Layout: {
        siderColor: "rgb(24,24,28)",
      },
      common: {
        bodyColor: "rgb(24,24,28)",
      },
      Card: {
        color: "rgb(24,24,28)",
      },
    },
    LightthemeOverrides: {
      Layout: {
        siderColor: "#fff",
      },
      common: {
        bodyColor: "#fff",
      },
      Card: {
        color: "fff",
      },
    },
  }),
  persist: true,
});
