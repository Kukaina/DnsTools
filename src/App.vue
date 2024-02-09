
<template>
<n-layout has-sider>
    <n-layout-sider bordered  >
        <Logo></Logo>
        <n-menu :options="Menuoptions" :on-update:value="func" />
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
import { NLayout,NMenu,NLayoutContent,NLayoutSider } from "naive-ui";
import{InformationCircle,SpeedometerSharp,SettingsSharp,WifiSharp}from "@vicons/ionicons5"
import { NIcon } from 'naive-ui'
import Logo from "./components/logo/index.vue"
import {h} from "vue"
import { useRouter } from "vue-router";
const Menuoptions=[
    {label:"Dns测速",key:"/",icon:renderIcon(SpeedometerSharp)},
    {label:"污染检测",key:"/pollute",icon:renderIcon(WifiSharp)},
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
.content{
    margin-left: 2%;
    margin-right: 2%;
}

</style>