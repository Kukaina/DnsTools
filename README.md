<div align=center><img src="https://pic.imgdb.cn/item/65cf0e309f345e8d039ae2a3.png"></div>

# 介绍
一款用于进行DNS测速和DNS污染检测的小工具，目前内置了122个常用DNS，支持`MacOS`，`Windows`，`Linux`三端（Win11，Win10可以稳定运行，其余自测），
并支持自动更新，使用Tauri+Vue3进行开发，体积小巧约10M左右（Windows端） 由于本软件基于Tauri开发，所以系统应拥有webview2运行时，一般系统都有，如果打不开软件的话可以尝试进行安装
https://developer.microsoft.com/zh-cn/microsoft-edge/webview2

# 使用方法
打开[release](https://github.com/Kukaina/DnsTools/releases/)
选择最新版进行下载即可

# 编译方法
* 1.安装环境
依赖环境包括nodejs(20)和rust环境
首先安装node.js环境
打开[Node.js](https://nodejs.org/en)安装即可
安装完成后请安装`yarn`作为包管理器
```
npm install --global yarn
```
然后是rust环境，请参照[文档](https://tauri.app/zh-cn/v1/guides/getting-started/prerequisites)进行安装
* 2.拉取项目
使用命令行或者网页下载项目源码
```
 git clone https://github.com/Kukaina/DnsTools.git
 cd DnsTools
```
* 3.安装依赖及编译
```
yarn
yarn tauri build
```
然后就可以在`.\DnsTools\src-tauri\target\release\bundle`下面找到编译产物
