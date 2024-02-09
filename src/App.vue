
<template>
<n-layout has-sider>
    <n-layout-sider bordered  >
        <span class="Logo_text">DnsTools</span>
        <n-menu :options="Menuoptions" :on-update:value="func"/>
    </n-layout-sider>
    <n-layout-content bordered>
        <div class="content">
            <RouterView></RouterView>
        </div>
    </n-layout-content>
</n-layout>
</template>

<script setup>
window.addEventListener('contextmenu', event => event.preventDefault());
import{InformationCircle,SpeedometerSharp,SettingsSharp,WifiSharp}from "@vicons/ionicons5"
import { NIcon } from 'naive-ui'
import {h} from "vue"
import { useRouter } from "vue-router";
const Menuoptions=[
    {label:"Dns测速",key:"/",icon:renderIcon(SpeedometerSharp)},
    {label:"污染检测",key:"/hijack",icon:renderIcon(WifiSharp)},
    {label:"关于",key:"/about",icon:renderIcon(InformationCircle)},
    {label:"设置",key:"/settings",icon:renderIcon(SettingsSharp)},
]
const router =useRouter()
function func(key,item){
    router.push(key)
}
function renderIcon (icon) {
  return () => h(NIcon, null, { default: () => h(icon) })
}
</script>

<style lang="scss" scoped>
.n-menu{
    .n-menu-item{
        width: 10px;
    }
}
.Logo_text{
    display: block;
    margin-top: 35%;
    margin-left: 12%;
    margin-bottom: 10%;
    font-size: 48px;
    font-weight: bold;
}
.content{
    margin-left: 2%;
    margin-right: 2%;
}

</style>