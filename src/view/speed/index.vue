<template>
  <div>
    <n-flex>
      <n-card title="介绍">
        这里你可以对常见的一百多个DNS进行测速，以便你选择出速度最快的DNS，提高上网体验，但需要注意的是，部分DNS有被劫持的可能，因此我们还有劫持检测功能，可以在替换完DNS后检测DNS是否被劫持,如果表格高度出现问题请按F5进行刷新
      </n-card>
      <n-button
        type="primary"
        @click="startTest(data.list)"
        :disabled="flag != 0"
      >
        {{ buttonText }}
      </n-button>
      <!-- 处理异步组件 -->
      <Suspense>
        <template #default>
          <n-data-table
            :columns="columns"
            :data="data.list"
            :max-height="tableHeight"
          />
        </template>
        <template #fallback> loading... </template>
      </Suspense>
    </n-flex>
  </div>
</template>

<script setup>
import { NFlex, NCard, NButton, NDataTable,useMessage } from "naive-ui";
import { onBeforeMount, ref,onMounted } from "vue";
import { invoke } from "@tauri-apps/api";
import { useSettingsStore } from "../../store/settings";
const settings=useSettingsStore()
const tableHeight = ref(window.innerHeight - 280);
const columns = [
  { title: "DNS地址", key: "IP" },
  { title: "DNS名称", key: "name" },
  { title: "DNS延迟", key: "delay" },
];
let data = ref([]);
let flag = ref(0);
let buttonText = ref("开始测速！！！");
onBeforeMount(async () => {
  try {
    data.value = await invoke("get_records", {
      params: { path: settings.filePath },
    });
  } catch (error) {
    useMessage().error("列表初始化出错，请检查文件设置 "+error)
  }
});
const startTest = async (list) => {
  flag.value = 1;
  buttonText.value = "正在测速中...";
  for (let i of list) {
    try {
      const response = await invoke("pings", { address: i.IP });
      i.delay = response + " ms";
    } catch (err) {
      i.delay = "请求超时";
      continue;
    }
  }
  flag.value = 3;
  buttonText.value = "正在排序中...";
  sortByDelay(list);
  flag.value = 0;
  buttonText.value = "已完成排序";
};

function sortByDelay(list) {
  // 自定义排序函数，确保字符排在数字后面
  function compareDelay(a, b) {
    // 尝试将 delay 转换为数字
    const delayA = parseFloat(a.delay) || Number.MAX_SAFE_INTEGER;
    const delayB = parseFloat(b.delay) || Number.MAX_SAFE_INTEGER;

    // 如果 delay 是数字，按数字排序
    if (!isNaN(delayA) && !isNaN(delayB)) {
      return delayA - delayB;
    }

    // 如果 delay 是字符，确保字符排在数字后面
    if (isNaN(delayA) && !isNaN(delayB)) {
      return 1; // a 排在 b 后面
    } else if (!isNaN(delayA) && isNaN(delayB)) {
      return -1; // a 排在 b 前面
    }

    // 如果 delay 都是字符，按字符串排序
    return a.delay.localeCompare(b.delay);
  }

  // 使用自定义排序函数对列表进行排序
  return list.sort(compareDelay);
}

</script>

<style lang="scss" scoped></style>
