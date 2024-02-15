
<template>
<n-layout has-sider>
    <n-layout-sider bordered >
        <NAffix :trigger-top="80" :top="0">
        <Logo></Logo>
        <Menu></Menu>
        </NAffix>

    </n-layout-sider>
    <n-layout-content bordered >
        <div class="content" >
            <RouterView></RouterView>
        </div>
    </n-layout-content>
</n-layout>

</template>

<script setup>
import Menu from "@/components/menu/index.vue"
import { NLayout,NLayoutContent,NLayoutSider,NAffix } from "naive-ui";
import Logo from "@/components/logo/index.vue"
</script>

<style lang="scss" scoped>
.n-menu{
    .n-menu-item{
        width: 10px;
    }
}
.n-affix{
    width: 270px;
    transition: top 10s;
}
.content{
    margin-left: 2%;
    margin-right: 2%;
    margin-top: 2%;
}

</style>