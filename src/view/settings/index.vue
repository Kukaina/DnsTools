<template>
<n-card title="Dns文件路径">
    <n-input-group>
        <n-button  type="primary" @click="reset"> 恢复默认 </n-button>
      <n-input :style="{width:'80%'}" v-model:value="settings.filePath" placeholder="请输入Json文件路径或点击右方选择" :on-blur="Check" />
      <n-button  type="primary" @click="SelectFile"> 选择文件 </n-button>
    </n-input-group>
</n-card>
</template>

<script setup>
import { useSettingsStore } from '../../store/settings';
import { NCard,NInput,NButton,NInputGroup,useMessage } from 'naive-ui';
import { dialog } from '@tauri-apps/api';
const settings=useSettingsStore()
async function SelectFile() {
  try {
    const selected = await dialog.open();
    settings.filePath=selected
  } catch (error) {
    useMessage().error("文件选择出错"+error)
  }
}
const reset=()=>{
    settings.$reset()
}
</script>

<style lang="scss" scoped>

</style>