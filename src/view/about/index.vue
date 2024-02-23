<template>
  <span class="Logo">DnsTools</span>
  <n-flex>
    <n-card title="简介" id="jianjie">
      Dnstools 是一个用 Tauri 框架开发的优秀 DNS
      工具，它结合了高性能和易用性。以下是该工具的一些显著特点：<br />
      轻量级和高效：利用 Tauri 框架的优势，Dnstools
      提供了一个相对于传统桌面应用更为轻量和高效的解决方案，减少了系统资源的占用。<br />
      DNS 测速：用户可以通过 Dnstools 测量各个 DNS
      服务器的响应时间，从而选择最快的服务器以优化网络性能。<br />
      污染检测：该工具能够检测 DNS 污染，帮助用户识别和避免潜在的 DNS 劫持和篡改风险。<br />
      用户友好的界面：尽管背后的技术复杂，Dnstools
      提供了一个简洁且直观的用户界面，使得用户易于操作和理解。<br />
      安全性：Tauri 框架的安全模型确保了前端代码在沙箱环境中运行，增强了应用程序的安全性。<br />
      跨平台兼容性：Tauri 使得 Dnstools 能够在不同的操作系统上运行，包括 Windows、macOS 和
      Linux，提供了良好的跨平台体验。<br />
      开源：DnsTools正式版完全开源，允许其他开发者和技术爱好者贡献代码，共同改进和增强工具的功能。<br />
      Dnstools
      通过这些特点，为网络用户和专业人士提供了一个实用的工具，能够帮助他们在保持网络性能的同时，确保他们的网络连接不受不必要的干扰。<br />
      <b>我才不会告诉你这些介绍是GPT写到呢..才..才不会呢</b>
    </n-card>
    <n-card title="开发人员" id="kaifa">
      <n-avatar round size="large" src="https://pic.imgdb.cn/item/65bdaf56871b83018a17cd21.webp" />
    </n-card>
    <n-card title="开源及反馈">
      <p>
        GIthub开源地址:<a
          target="_blank"
          href="https://github.com/Kukaina/DnsTools/"
          class="githublink"
        >
          <p>https://github.com/Kukaina/DnsTools/</p>
        </a>
      </p>
      <br />
      <p>
        反馈地址:<a
          href="https://github.com/Kukaina/DnsTools/issues/new"
          target="_blank"
          class="githublink"
          ><p>https://github.com/Kukaina/DnsTools/issues/new</p></a
        >
      </p>
    </n-card>
    <n-card title="更新日志" id="log">
      <h2>V1.2.3_240223</h2>
      <span>增加了一些华而不实(bushi)的功能(其实是为了试试新的Vscode主题好不好看才写的)</span>
      <n-divider />
      <h2>V1.2.2_240220</h2>
      <span>支持Ctrl +/- 进行缩放</span>
      <n-divider />
      <h2>V1.2.1_240219</h2>
      <span>新增IPV6 DNS列表，增加消息提醒，优化污染检测功能</span>
      <n-divider />
      <h2>V1.2.0_240218</h2>
      <span
        >新增精简版DNS列表，修复切换页面时测速消失的问题，支持导出为CSV，新增GIthub链接,新增右键复制DNS地址功能</span
      >
      <n-divider />
      <h2>V1.1.0_240215_beta</h2>
      <span>新增设置界面，优化页面结构,新增错误检测</span>
      <h2>V1.0.3.240209_beta</h2>
      <span>修复头像无法显示的Bug</span>
      <n-divider />
      <h2>V1.0.2.240209_beta</h2>
      <span>优化表格显示，调整字体</span>
      <n-divider />
      <h2>V1.0.1.240209_beta</h2>
      <span>增加MacOS，以及Linux支持，并添加自动更新功能</span>
      <n-divider />
      <h2>V1.0.0.240208_beta</h2>
      <span>使用tauri进行重构并完成基础功能，并增加DNS污染检测功能</span>
      <n-divider />
      <h2>V0.2.0.240131_beta</h2>
      <span>增加排序以及字体染色功能</span>
      <n-divider />
      <h2>V0.1.1.240130_beta</h2>
      <span>优化界面布局，改善体验</span>
      <n-divider />
      <h2>V0.1.0.240129_beta</h2>
      <span>使用Qt完成DNS排序功能</span>
    </n-card>
  </n-flex>
</template>

<script setup>
  import { NFlex, NCard, NDivider, NAvatar } from "naive-ui";
</script>

<style lang="scss" scoped>
  .Logo {
    display: block;
    margin-left: 40%;
    font-weight: bold;
    font-size: 48px;
    margin-top: 15%;
  }

  h2 {
    font-weight: bold;
    margin: 10px 10px;
  }

  .githublink {
    text-decoration: none;
    border-radius: 5px;
    color: var(--primary-color);
    background-color: transparent; // 默认背景透明
    transition: background-color 0.3s ease, color 0.3s ease, padding 0.3s ease; // 过渡效果
    p {
      display: inline;
    }

    &:hover,
    &:focus {
      color: white; // 鼠标悬停和聚焦时文字颜色变为白色
      background-color: rgb(66, 90, 239); // 鼠标悬停和聚焦时背景颜色变化
      border-radius: 5px; // 圆角
      padding: 2px 8px; // 文字的上下左右内边距
      text-decoration: none; // 移除文本下划线
    }
  }
</style>
