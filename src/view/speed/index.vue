<template>
  <div>
    <n-flex>
      <n-card title="介绍">
        这里你可以对常见的一百多个DNS进行测速，以便你选择出速度最快的DNS，提高上网体验，但需要注意的是，部分DNS有被污染的可能，因此我们还有污染检测功能，可以在替换完DNS后检测DNS是否被污染,如果表格高度出现问题请按F5进行刷新，由于某些DNS服务器禁ping，所以可能会导致测试失败，目前正在寻找解决方案(DNS查询会导致缓存问题)
      </n-card>
      <n-alert
        title="你正在使用IPV6 DNS"
        type="warning"
        style="display: block; width: 100%"
        :bordered="false"
        closable
        v-if="!settings.firstwarn && settings.filePath == './DnsList.V6.json'"
        @close="settings.firstwarn = true"
      >
        测量IPV6 DNS时必须保证你本机已经拥有了IPV6地址，如果没有将测试失败<br />
        可以尝试前往<a
          href="https://test-ipv6.com/"
          target="_blank"
          id="testlink"
          >测试地址</a
        >进行测试<br />
        或终端输入 ipconfi /all 进行查看
      </n-alert>
      <n-button
        type="primary"
        @click="Start"
        :disabled="speedstore.teststatus != 0"
      >
        {{ speedstore.buttonText }}
      </n-button>
      <n-button
        type="primary"
        @click="exportSorterAndFilterCsv"
        :disabled="speedstore.teststatus != 0"
        >导出CSV</n-button
      >
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
import { NFlex, NCard, NButton, NDataTable, NDropdown, NAlert } from "naive-ui";
import { onBeforeMount, ref, reactive } from "vue";
import { useSpeedStore } from "../../store/speed";
import { useSettingsStore } from "../../store/settings";
import {useToast} from 'vue-toast-notification';
const showDropdown = ref(false);
const dropdownCoords = reactive({ x: 0, y: 0 });
const selectedRowData = ref(null);
const settings = useSettingsStore();
const speedstore = useSpeedStore();
const tableHeight = ref(window.innerHeight - 300);
const columns = [
  { title: "DNS地址", key: "IP" },
  { title: "DNS名称", key: "name" },
  { title: "DNS延迟", key: "delay" },
];
onBeforeMount(() => {
  speedstore.iniData();
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

const tableRef = ref();
const exportSorterAndFilterCsv = () =>
  tableRef.value?.downloadCsv({
    fileName: "测速结果",
    keepOriginalData: false,
  });
const Start = () => {
  speedstore.startTest();
};
//实时排序
speedstore.$subscribe((mutation, state) => {
  sortByDelay(speedstore.data.list);
});
//监听刷新事件，防止出现No data
window.addEventListener("beforeunload", (event) => {
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
    onContextmenu: (event) => onRowRightClick(event, rowData),
  };
};

const dropdownOptions = [{ label: "复制DNS地址", key: "copyDelay" }];

function handleDropdownSelect(key, option) {
  if (key === "copyDelay" && selectedRowData.value) {
    const delayToCopy = selectedRowData.value.IP;
    copyToClipboard(delayToCopy);
  }
  showDropdown.value = false;
}
const copyToClipboard = (text) => {
  navigator.clipboard
    .writeText(text)
    .then(() => {
      useToast().open({
        message:"成功复制IP："+text,
        type:"success",
        position:"top-right"
      })
    })
    .catch((err) => {
      useToast().open({
        message:"复制出错："+err,
        type:"error",
        position:"top-right"
      })
    });
};
</script>

<style lang="scss" scoped></style>
