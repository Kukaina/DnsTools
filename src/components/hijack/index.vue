<!--  -->
<template>
<n-flex>
    <n-card title="介绍">
本工具使用本机DNS与8.8.8.8 DNS解析同一域名，根据结果是否存在差异来判断DNS污染情况，本工具只可以简单判断污染情况并不可作为参考依据，如发现判断错误的情况请以实际为准,误判常见原因包括但不限于缓存效应，DNS拦截，网络路由问题，程序bug(bushi)等等
</n-card>
<n-input type="text" size="large" placeholder="请输入域名进行测试：" v-model:value="text"/>
<n-button @click="startTest">开始测试</n-button>
<n-card title="测试结果">{{ testResult }}</n-card>
</n-flex>
</template>

<script setup>
import { ref } from 'vue';
import {invoke} from '@tauri-apps/api'
const text=ref()
const testResult=ref("等待测试中")
const startTest=()=>{
    invoke('check_dns_pollution',{domain:text.value}).then((result)=>{
        try{
            if (result){
            testResult.value="您的DNS可能遭到了污染"
        }
        else if(!result){
            testResult.value="您的DNS并未遭到污染"
        }
        }
        catch(err){
            testResult.value="测试出现了错误，请稍后重试或尝试联系开发者"
            console.log(err);
        }
    })
}
</script>

<style lang="scss" scoped>

</style>