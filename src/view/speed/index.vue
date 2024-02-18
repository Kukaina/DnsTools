<template>
  <div>
    <n-flex>
      <n-card title="介绍">
        这里你可以对常见的一百多个DNS进行测速，以便你选择出速度最快的DNS，提高上网体验，但需要注意的是，部分DNS有被劫持的可能，因此我们还有劫持检测功能，可以在替换完DNS后检测DNS是否被劫持,如果表格高度出现问题请按F5进行刷新，由于某些DNS服务器禁ping，所以可能会导致测试失败，目前正在寻找解决方案(DNS查询会导致缓存问题)
      </n-card>
      <n-button
        type="primary"
        @click="Start"
        :disabled="speedstore.teststatus != 0"
      >
        {{ speedstore.buttonText }}
      </n-button>
      <n-button type="primary" @click="exportSorterAndFilterCsv" :disabled="speedstore.teststatus !=0">导出CSV</n-button>
      <!-- 处理异步组件 -->
      <Suspense>
        <template #default>
            <n-data-table
            :columns="columns"
            :data="speedstore.data.list"
            :max-height="tableHeight"
            ref="tableRef"
            :row-props="rowProps"
          />
        </template>
        <template #fallback> loading... </template>
      </Suspense>
      <n-dropdown
  v-model:show="showDropdown"
  :x="dropdownCoords.x"
  :y="dropdownCoords.y"
  :options="dropdownOptions"
  @select="handleDropdownSelect"
/>
    </n-flex>
  </div>
</template>

<script setup>
import { NFlex, NCard, NButton, NDataTable,NDropdown,useMessage } from "naive-ui";
import { onBeforeMount, ref,reactive } from "vue";
import {useSpeedStore}from "../../store/speed"
import { useSettingsStore } from "../../store/settings";

const showDropdown = ref(false);
const dropdownCoords = reactive({ x: 0, y: 0 });
const selectedRowData = ref(null);
const settings=useSettingsStore()
const speedstore=useSpeedStore()
const tableHeight = ref(window.innerHeight - 300);
const columns = [
  { title: "DNS地址", key: "IP" },
  { title: "DNS名称", key: "name" },
  { title: "DNS延迟", key: "delay" },
];
onBeforeMount(()=>{
speedstore.iniData()
});



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

const tableRef=ref();
const exportSorterAndFilterCsv = () => tableRef.value?.downloadCsv({
      fileName: "测速结果",
      keepOriginalData: false
    });
const Start=()=>{
  speedstore.startTest();
}
//实时排序
speedstore.$subscribe((mutation,state)=>{
  sortByDelay(speedstore.data.list);
})
//监听刷新事件，防止出现No data
window.addEventListener('beforeunload',(event)=>{
  // 清除sessionStorage
  sessionStorage.clear();
});

function onRowRightClick(event, rowData) {
  event.preventDefault();
  showDropdown.value = true;
  dropdownCoords.x = event.clientX;
  dropdownCoords.y = event.clientY;
  selectedRowData.value = rowData;
}


const rowProps = (rowData) => {
  return {
    onContextmenu: (event) => onRowRightClick(event, rowData)
  };
};

const dropdownOptions = [
{ label: '复制DNS地址', key: 'copyDelay' },
];

function handleDropdownSelect(key, option) {
  if (key === 'copyDelay' && selectedRowData.value) {
    const delayToCopy = selectedRowData.value.IP;
    copyToClipboard(delayToCopy);
  }
  showDropdown.value = false;
}
const copyToClipboard = (text) => {
  navigator.clipboard.writeText(text).then(() => {
    console.log('Text copied to clipboard');
  }).catch(err => {
    console.error('Failed to copy text to clipboard', err);
  });
};
</script>

<style lang="scss" scoped></style>
