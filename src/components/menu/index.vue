<template>
    <div>
        <n-menu :options="menulist" :on-update:value="jumpTo" :default-value="path" />
    </div>
</template>

<script setup>
import { NMenu, NIcon } from 'naive-ui';
import { h, ref } from "vue"
import { useRouter,useRoute } from 'vue-router';
import { onMounted } from 'vue';

const path=useRoute().path;
const router = useRouter();
const menulist = ref([]);
// 动态引入并渲染Icon
async function renderIcon(icon) {
  const { [icon]: iconComp } = await import("@vicons/ionicons5");
  return () => h(NIcon, null, { default: () => h(iconComp) });
}
//根据路由生成Menu配置
onMounted(async () => {
  const menuItemsPromises = router.options.routes.map(async item => {
    const iconComponent = await renderIcon(item.meta.icon);
    return {
      label: item.meta.label,
      key: item.path,
      icon: iconComponent
    };
  });

  menulist.value = await Promise.all(menuItemsPromises);
});
//路由跳转
function jumpTo(key, item) {
  router.push(key);
}
</script>
<style lang="scss" scoped>

</style>