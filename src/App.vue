<template>
  <n-layout has-sider>
    <n-layout-sider bordered>
      <NAffix :trigger-top="80" :top="0">
        <Logo></Logo>
        <Menu></Menu>
      </NAffix>
    </n-layout-sider>
    <n-layout-content bordered>
      <div class="content">
        <RouterView></RouterView>
      </div>
      <div class="zoom" style="zoom: 1 !important;">
        {{ "缩放 "+settings.zoom*100+"%"}}
      </div>
    </n-layout-content>
  </n-layout>
</template>

<script setup>
import Menu from "@/components/menu/index.vue";
import { NLayout, NLayoutContent, NLayoutSider, NAffix } from "naive-ui";
import { useSettingsStore } from "./store/settings";
import { ref } from "vue";
import Logo from "@/components/logo/index.vue";
const settings=useSettingsStore()
document.body.style.zoom=settings.zoom
document.addEventListener('keydown', function(event) {
    // 检查是否同时按下了Ctrl（Cmd）和'+'或'-'键
    if (event.ctrlKey || event.metaKey) { // 在Mac上使用metaKey代替ctrlKey
        const zoomIn = event.key === '=';
        const zoomOut = event.key === '-';
        if (zoomIn || zoomOut) {
            // 防止默认行为，比如在输入框中输入'+'或'-'
            event.preventDefault();
            // 放大或缩小
            if (zoomIn) {
              if(settings.zoom < 1.7){
                settings.zoom = parseFloat((settings.zoom+0.1).toFixed(3)); // 放大10%
              }
            } else if (zoomOut) {
              if(settings.zoom > 0.7){
                settings.zoom = parseFloat((settings.zoom-0.1).toFixed(3)); // 缩小10%
              }
            }
            // 设置新的缩放级别
            document.body.style.zoom = settings.zoom;
        }
    }
});
settings.$subscribe((matation,state)=>{
    console.log(settings.zoom * 100);
    let zoomElement = document.querySelector('.zoom');
    zoomElement.classList.add('visible');
    setTimeout(function() {
      zoomElement.classList.remove('visible');
    }, 1000);
})
</script>

<style lang="scss" scoped>
.n-menu {
  .n-menu-item {
    width: 10px;
  }
}
.n-affix {
  width: 270px;
  transition: top 10s;
}
.content {
  margin-left: 2%;
  margin-right: 2%;
  margin-top: 2%;
}
.zoom {
  position: absolute;
  top: 0;
  right: 0;
  margin: 2px 2px;
  background-color: rgb(158,158,158);
  opacity: 0;
  border-radius: 3px;
  color: white;
  transition: opacity 0.5s linear;
  visibility: hidden;

  &.visible {
    opacity: 0.7;
    visibility: visible;
  }
}

</style>
