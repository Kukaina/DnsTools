import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api";
import { useSettingsStore } from './settings';
export const useSpeedStore = defineStore('speed', {
  state: () => ({
    data:[],
    teststatus:0,
    buttonText:"开始测量 ! ! !"
  }),
  getters: {
  },
  actions: {
    async iniData() {
      const settings = useSettingsStore();
      if(!sessionStorage.getItem("Ini")){
            try {
                this.data = await invoke("get_records", {
                  params: { path: settings.filePath },
                });
                sessionStorage.setItem("Ini",1)
              } catch (error) {
                console.log(error);
              }
        }
    },
    async startTest(){
        this.teststatus = 1;
        this.buttonText = "正在测速中...";
        for (let i of this.data.list) {
          try {
            i.delay="测量中..."
            const response = await invoke("pings", { address: i.IP });
            i.delay = response + " ms";
      
          } catch (err) {
            i.delay = "请求超时";
            console.log({"当前索引:":i,"错误:":err});
            continue;
          }
        }
        this.teststatus = 0;
        this.buttonText = "开始测量";
      },
  },
});