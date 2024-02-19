<template>
  <!-- 还没写完 -->
  <!-- <n-card title="更新DNS文件">
    <n-button type="primary" style="width: 40%; margin-right: 10%">
      完整DNS列表
    </n-button>
    <n-button type="primary" style="width: 40%; margin-top: 20px">
      精简版DNS列表
    </n-button>
  </n-card> -->
  <n-card title="自定义DNS文件路径">
    <n-space style="width: 100%">
      <n-input-group style="width: 100%">
        <n-button type="primary" @click="reset"> 恢复默认 </n-button>
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
