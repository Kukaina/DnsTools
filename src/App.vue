<template>
  <NConfigProvider
    :theme="settings.theme"
    :theme-overrides="
      settings.theme === null ? settings.LightthemeOverrides : settings.DarkthemeOverrides
    "
  >
    <n-layout has-sider style="height: 100vh">
      <n-layout-sider bordered class="blur-background" style="backdrop-filter: blur(10px)">
        <NAffix :trigger-top="80" :top="0">
          <Logo></Logo>
          <Menu></Menu>
          <themeButton></themeButton>
        </NAffix>
      </n-layout-sider>
      <n-layout-content bordered>
        <div class="content">
          <RouterView></RouterView>
        </div>
        <div class="zoom" style="zoom: 1 !important">
          {{ "缩放 " + settings.zoom * 100 + "%" }}
        </div>
      </n-layout-content>
    </n-layout>
  </NConfigProvider>
</template>

<script setup>
  import Menu from "@/components/menu/index.vue";
  import themeButton from "@/components/themeButton/index.vue";
  import { onMounted, onBeforeMount } from "vue";
  import { useToast } from "vue-toast-notification";
  import {
    NLayout,
    NLayoutContent,
    NLayoutSider,
    NAffix,
    NConfigProvider,
    darkTheme,
  } from "naive-ui";
  import { useSettingsStore } from "./store/settings";
  import { ref } from "vue";
  import Logo from "@/components/logo/index.vue";
  const settings = useSettingsStore();
  try {
    document.body.style.zoom = settings.zoom;
  } catch (error) {
    useToast().open({
      message: "初始化出错：" + error,
      type: "error",
      position: "top-right",
    });
  }
  document.addEventListener("keydown", function (event) {
    // 检查是否同时按下了Ctrl（Cmd）和'+'或'-'键
    if (event.ctrlKey || event.metaKey) {
      // 在Mac上使用metaKey代替ctrlKey
      const zoomIn = event.key === "=";
      const zoomOut = event.key === "-";
      if (zoomIn || zoomOut) {
        // 防止默认行为，比如在输入框中输入'+'或'-'
        event.preventDefault();
        // 放大或缩小
        if (zoomIn) {
          if (settings.zoom < 1.7) {
            settings.zoom = parseFloat((settings.zoom + 0.1).toFixed(3)); // 放大10%
          }
        } else if (zoomOut) {
          if (settings.zoom > 0.7) {
            settings.zoom = parseFloat((settings.zoom - 0.1).toFixed(3)); // 缩小10%
          }
        }
        // 设置新的缩放级别
        document.body.style.zoom = settings.zoom;
      }
    }
  });
  settings.$subscribe((matation, state) => {
    console.log(settings.zoom * 100);
    let zoomElement = document.querySelector(".zoom");
    try {
      zoomElement.classList.add("visible");
    } catch (error) {
      useToast().open({
        message: "初始化出错：" + error,
        type: "error",
        position: "top-right",
      });
    }
    setTimeout(function () {
      zoomElement.classList.remove("visible");
    }, 1000);
  });
  onMounted(() => {
    document.querySelector(
      ".n-layout-sider"
    ).style.backdropFilter = `blur(${settings.filternum}px)`;
    document.querySelector(
      ".n-layout-sider"
    ).style.webkitBackdropFilter = `blur(${settings.filternum}px)`;
  });
  //检检查设置
  onBeforeMount(() => {
    //如果没检测到设置项
    if (!localStorage.getItem("settings")) {
      settings.$reset();
      useToast().open({
        message: "检测到设置项出错，已重置设置项",
        type: "success",
        position: "top-right",
      });
    }
  });
</script>

<style lang="scss" scoped>
  // .blur-background {
  //   backdrop-filter: blur(10px); /* 设置背景模糊的半径 */
  //   -webkit-backdrop-filter: blur(10px); /* Safari 兼容性 */
  //   background-color: rgba(116, 99, 99, 0.075); /* 设置背景颜色和透明度，以确保模糊效果可见 */
  // }
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
    background-color: rgba(117, 110, 110, 0.445);
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
