import { defineStore } from 'pinia'


export const useSettingsStore = defineStore('settings', {
    state: () => ({
        filePath: "./DnsList.json"
    }),
    persist:true

})