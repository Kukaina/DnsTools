<template>
  <n-flex>
    <n-card title="介绍">
      本工具使用本机DNS与8.8.8.8
      DNS解析同一域名，根据结果是否存在差异来判断DNS污染情况，本工具只可以简单判断污染情况并不可作为参考依据，如发现判断错误的情况请以实际为准,误判常见原因包括但不限于缓存效应，DNS拦截，网络路由问题，程序bug(bushi)等等
    </n-card>
    <n-select
      v-model:value="selectValue"
      filterable
      tag
      :options="pullutestore.options"
      @update:value="handleUpdate"
      placeholder="请选择或输入域名"
    />

    <n-button type="primary" @click="startTest">开始测试</n-button>
    <n-button type="primary" @click="reset">清空历史记录</n-button>
    <n-card title="测试结果">
      <n-table>
        <thead>
          <tr>
            <th>公共DNS解析结果</th>
            <th>本机DNS解析结果</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(publicIp, index) in data.public_response" :key="index">
            <td>{{ publicIp }}</td>
            <td>{{ data.system_response[index] }}</td>
          </tr>
          <tr>
            <td>最终测试结果</td>
            <td v-if="data.all_system_ips_not_in_public == True">
              您的DNS可能遭到了污染
            </td>
            <td v-if="!data.all_system_ips_not_in_public">
              您的DNS并未遭到污染
            </td>
          </tr>
        </tbody>
      </n-table>
    </n-card>
  </n-flex>
</template>

<script setup>
import { NFlex, NCard, NButton, NTable, NSelect } from "naive-ui";
import {useToast} from 'vue-toast-notification';
import { ref } from "vue";
import { usePulluteStore } from "../../store/pullute";
import { invoke } from "@tauri-apps/api";
const pullutestore = usePulluteStore();
const selectValue = ref(null);
const data = ref({
  all_system_ips_not_in_public: "待测试",
  public_response: ["待测试"],
  system_response: ["待测试"],
});
const testResult = ref("等待测试中");
const startTest = () => {
  invoke("check_dns_pollution", { domain: selectValue.value })
    .then((result) => {
      try {
        if (result) {
          data.value = result;
        } else if (!result) {
          testResult.value = "您的DNS并未遭到污染";
        }
      } catch (err) {
        testResult.value = "测试出现了错误，请稍后重试或尝试联系开发者";

      }
    })
    .catch((err) => {
      useToast().open({
        message:"测试出错："+err,
        type:"error",
        position:"top-right"
      })
    });
};
const handleUpdate = (value) => {
  pullutestore.addHistoryInput(value);
};
const reset = () => {
  pullutestore.$reset();
};
</script>

<style lang="scss" scoped></style>
