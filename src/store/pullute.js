import { defineStore } from 'pinia'

export const usePulluteStore = defineStore('pullute', {
    state: () => ({
        options:[{
            type:"group",
            label:"蓝奏云",
            key:"蓝奏云",
            children:[
                {label:"lanzoui.com",value:"lanzoui.com"},
                {label:"lanzoul.com",value:"lanzoul.com"},
                {label:"lanzoux.com",value:"lanzoux.com"},
                {label:"lanzouw.com",value:"lanzouw.com"},
            ]
        },
            {label:"推特",value:"twitter.com"},
            {label:"谷歌",value:"google.com"},
            {label:"Gmail",value:"gmail.com"},
            {label:"FaceBook",value:"facebook.com"},
            {label:"Instagram",value:"instagram.com"},
            {label:"Netflix",value:"netflix.com"},
            {
                type:"group",
                label:"历史输入",
                value:"历史输入",
                children:[]
            }
]}),
    persist:true,
    actions: {
        addHistoryInput(inputValue) {
            const isValueUnique = !this.options.some(option => {
              // 检查是否为 group 类型并搜索其 children
              if (option.type === 'group') {
                return option.children.some(child => child.value === inputValue);
              }
              // 不是 group 类型，直接比对 value
              return option.value === inputValue;
            });
        
            if (isValueUnique) {
              const historyGroup = this.options.find(option => option.label === '历史输入');
              if (historyGroup) {
                const newItem = { label: inputValue, value: inputValue };
                historyGroup.children.push(newItem);
              }
            }
          }
      }

})