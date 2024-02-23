<template>
  <n-card title="自定义DNS文件路径">
    <n-space style="width: 100%">
      <n-input-group style="width: 100%">
        <n-input
          :style="{ width: '80%' }"
          v-model:value="settings.filePath"
          placeholder="请输入Json文件路径或点击右方选择"
        />
        <n-button type="primary" @click="SelectFile"> 选择文件 </n-button>
      </n-input-group>
    </n-space>
    <n-select
      v-model:value="settings.filePath"
      :options="options"
      placeholder="选择DNS列表模式"
      style="margin-top: 20px"
    />
  </n-card>
  <n-card>
    <n-button type="primary" @click="reset" style="width: 100%"> 恢复默认 </n-button>
  </n-card>
  <n-card title="外观设置">
    <n-card title="侧边栏设置">
      <n-card title="侧边栏颜色(浅色)">
        <n-color-picker
          :default-value="settings.LightthemeOverrides.Layout.siderColor"
          v-model:value="settings.LightthemeOverrides.Layout.siderColor"
        ></n-color-picker>
      </n-card>
      <n-card title="侧边栏颜色(深色)">
        <n-color-picker
          :default-value="settings.DarkthemeOverrides.Layout.siderColor"
          v-model:value="settings.DarkthemeOverrides.Layout.siderColor"
        ></n-color-picker>
      </n-card>
      <n-card title="侧边栏模糊度(刷新生效)">
        <n-input-number
          v-model:value="settings.filternum"
          :default-value="settings.filternum"
        ></n-input-number>
      </n-card>
    </n-card>
    <n-card title="背景设置">
      <n-card title="背景图片设置">
        <n-input v-model:value="settings.backgroundimg" placeholder="请输入链接地址"></n-input>
      </n-card>
      <n-card title="背景颜色设置(浅色)">
        <n-color-picker
          v-model:value="settings.LightthemeOverrides.common.bodyColor"
          :default-value="settings.LightthemeOverrides.common.bodyColor"
        ></n-color-picker>
        <n-card title="背景颜色设置(深色)">
          <n-color-picker
            v-model:value="settings.DarkthemeOverrides.common.bodyColor"
            :default-value="settings.DarkthemeOverrides.common.bodyColor"
          ></n-color-picker>
        </n-card>
      </n-card>
      <n-card title="卡片颜色设置(浅色)">
        <n-color-picker
          v-model:value="settings.LightthemeOverrides.Card.color"
          :default-value="settings.LightthemeOverrides.Card.color"
        ></n-color-picker>
      </n-card>
      <n-card title="卡片颜色设置(深色)">
        <n-color-picker
          v-model:value="settings.DarkthemeOverrides.Card.color"
          :default-value="settings.DarkthemeOverrides.Card.color"
        ></n-color-picker>
      </n-card> </n-card
  ></n-card>
</template>

<script setup>
  import { useSettingsStore } from "../../store/settings";
  import {
    NCard,
    NInput,
    NButton,
    NInputGroup,
    useMessage,
    NSelect,
    NSpace,
    NColorPicker,
    NInputNumber,
  } from "naive-ui";
  import { dialog, invoke } from "@tauri-apps/api";
  import { ref } from "vue";
  const settings = useSettingsStore();
  async function SelectFile() {
    try {
      const selected = await dialog.open();
      settings.filePath = selected;
    } catch (error) {
      useMessage().error("文件选择出错" + error);
    }
  }
  const reset = () => {
    settings.$reset();
  };
  const options = [
    { label: "完整DNS", value: "./DnsList.json" },
    { label: "精简DNS", value: "./DnsList.min.json" },
    { label: "IPV6 DNS", value: "./DnsList.V6.json" },
  ];
</script>

<style lang="scss" scoped></style>
