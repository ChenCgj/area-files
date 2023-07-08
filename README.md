# Area Files

## 简介

该软件主要提供了局域网内不同的可互相通信的存储设备的文件共享功能，便于在不同存储设备上交换信息。其主要有以下三个功能：

1. 文件共享，包括共享文件夹配置，上传共享文件信息，浏览、下载共享文件，配置共享文件权限
2. 流媒体服务，每个用户都可以作为内容的提供者同时负责存储流媒体内容，网络内容无需集中在一台服务器上（由于中间服务器软件开发尚未完成，只能提供局域网内部的服务）
3. 控制信息传输，通过该软件控制局域网内的设备（尚未实现）

## 各个文件夹内容介绍

- `client-core` 存放客户端后台核心程序，用于处理局域网内不同主机的请求，同时处理用户通过客户端发来的请求，该程序还负责同中间服务器通信，提供对局域网外的服务
- `client` 存放客户端程序，提供命令行界面或者图形界面，向客户端后台核心程序发送请求，并将请求结果展现给用户，属于用户界面程序
- `server` 存放中间服务器程序，提供不同局域网间的文件中转，信息中转
- `common` 存放公共组件库，提供给服务器程序，客户端后台核心程序，客户端用户界面程序使用
- `experiment` 存放原型
- `docs` 存放设计文档（**请把代码库 clone 到本地以便 markdown 正常解析图片链接**）

## 构建

- `client-core` 中存放的 `area_files_client_core` 目录下是使用 rust 编写的后台核心程序，可以使用 `cargo build` 构建
- `client` 下的 `AreaFilesClient` 目录下是使用 kotlin 编写的跨端程序，可以在桌面环境或者Android上运行，采用 gradle 进行构建
- `server` 下的 `area_files_server` 目录下是使用 rust 编写的中间服务器程序，可以使用 `cargo build` 构建（tip：可以利用 GitHub 的 Action 进行 CI/CD）
- `experiment` 下的原型程序可以在 Linux 下使用 `make` 构建

## 使用教程

这套软件的中间服务器，客户端后台核心程序，客户端用户界面程序相互独立，可以各自替换，只要遵循文档中的协议就可以正常使用。

### 软件配置

在各个软件下有个 `config` 目录，里面有个 `config.json` 文件配置了软件监听的IP以及端口号，文件的保存地址等等，具体参数配置见 `docs` 目录下的详细设计文档

客户端后台核心程序安装在局域网内的所有主机上，客户端用户界面程序根据需要选择性安装，中间服务器程序需要一台具有公网IP的服务器部署



以下部分属于experiment部分README（可能已经过时），experiment文件夹内是该软件的原型，用纯C编写，只能在Linux平台上执行。


## Introduction

The `Area Files` is a file sharing system. Users can use this system to exchange any file between different hosts.

Althought it is designed to work in local area network, it also can share files between two hosts in different local area network or wide area network by using the transit server.

## Functions

1. list all files you can get in network
2. share you files by moving your files to `shareFiles` directory
3. download the file you choose
4. user registers on the server to provide the information of user's sharing file on the server
5. set permission of your files to prevent some hosts or users to get your share files
6. get shared file on the server if user have the right to the file

## Configure

configure the ip and port of the client/server (not finish).

## Build

***This is a temp directory. It's not the final software***

```shell
cd experiment
make
```

## usage

### client

- start client

   1. create the share directory `shareFiles` and download directory `downloads`
   2. launch the client: `build/client/client`

- some client command

   1. `list-my`
   
      list all shared files of yourself
   
   2. `fresh`
   
      get information of shared files in network
   
   3. `list-all`
   
      list the information got by `fresh`
   
   4. `download xxx`
   
      download the file xxx

   5. `exit`

      exit the program


### server

not finish

