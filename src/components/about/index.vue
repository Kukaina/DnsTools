<!--  -->
<template>
    <!-- <div>
        DNSTools 是一款由个人开发者Kuka（一位初中生）独立开发的网络工具，它展示了年轻开发者在技术领域的创新能力和对用户体验的深刻理解。这款工具不仅具备基本的DNS测速功能，还集成了网络信息展示和DNS劫持检测等实用特性，为用户提供了一个全面的网络诊断平台。

主要特点：

DNS性能优化：DNSTools 通过精确的测速功能，帮助用户找到最快的DNS服务器，从而提升网络浏览和应用加载速度。

网络状态监控：工具提供了实时的网络状态信息，包括IP地址、子网掩码、默认网关等，让用户对自己的网络环境有更清晰的认识。

安全防护：DNS劫持检测功能能够及时发现并警告用户可能存在的安全威胁，保护用户免受恶意攻击。

用户友好设计：尽管开发者Kuka年纪轻轻，但DNSTools 的界面设计简洁明了，易于操作，适合所有年龄段的用户。

跨平台兼容性：工具支持主流操作系统，确保用户在不同的设备上都能享受到相同的服务。

快速高效：利用Tauri框架的优势，DNSTools 在执行任务时表现出快速响应，为用户提供流畅的操作体验。
    </div> -->
    <!-- <n-grid>
        <n-gi span="20"> -->
            <span class="Logo">DnsTools</span>
            <n-flex>
                <n-card title="简介" id="jianjie">
                Dnstools 是一个用 Tauri 框架开发的优秀 DNS 工具，它结合了高性能和易用性。以下是该工具的一些显著特点：<br>
轻量级和高效：利用 Tauri 框架的优势，Dnstools 提供了一个相对于传统桌面应用更为轻量和高效的解决方案，减少了系统资源的占用。<br>
DNS 测速：用户可以通过 Dnstools 测量各个 DNS 服务器的响应时间，从而选择最快的服务器以优化网络性能。<br>
污染检测：该工具能够检测 DNS 污染，帮助用户识别和避免潜在的 DNS 劫持和篡改风险。<br>
用户友好的界面：尽管背后的技术复杂，Dnstools 提供了一个简洁且直观的用户界面，使得用户易于操作和理解。<br>
安全性：Tauri 框架的安全模型确保了前端代码在沙箱环境中运行，增强了应用程序的安全性。<br>
跨平台兼容性：Tauri 使得 Dnstools 能够在不同的操作系统上运行，包括 Windows、macOS 和 Linux，提供了良好的跨平台体验。<br>
开源：DnsTools正式版将开源，允许其他开发者和技术爱好者贡献代码，共同改进和增强工具的功能。<br>
Dnstools 通过这些特点，为网络用户和专业人士提供了一个实用的工具，能够帮助他们在保持网络性能的同时，确保他们的网络连接不受不必要的干扰。
            </n-card>
            <n-card title="开发人员" id="kaifa">
                <n-avatar
      round
      size="large"
      src="https://pic.imgdb.cn/item/65bdaf56871b83018a17cd21.webp"
    />
            </n-card>
            <n-card title="反馈方式" id="fankui">
            QQ: 2358755283<br>
            Email: 2358755283@qq.com    
            </n-card>
            <n-card title="更新日志" id="log">
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
        <!-- </n-gi>
        <n-gi span="4">
            <n-anchor
            affix
      listen-to=".document-scroll-container"
      :trigger-top="24"
      :top="88"
      style="z-index: 1"
      :bound="24"
            >
    <n-anchor-link title="简介" href="#jianjie" />
    <n-anchor-link title="开发人员" href="#kaifa" />
    <n-anchor-link title="反馈方式" href="#fankui" />
    <n-anchor-link title="更新日志" href="#log" />
  </n-anchor>
        </n-gi>
    </n-grid> -->
</template>

<script setup>

</script>

<style lang="scss" scoped>
.Logo{
    display: block;
    margin-left: 40%;
    font-weight: bold;
    font-size: 48px;
    margin-top: 15%;
}
h2{
    font-weight:bold;
    margin: 10px 10px;
}
</style>