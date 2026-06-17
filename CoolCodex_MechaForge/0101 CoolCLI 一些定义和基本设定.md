


我们管采用 Cool 架构的 CLI， 不管是被 Cool架构 深度改造过的，还是纯粹从零开始开发的 ，都叫做 **CoolCLI**

被 Cool架构深度改造过的 叫 **CoolCodex**
我是用 Cool架构 从零创造的第一个 CLI叫做  **CodeElemental**



我们通过一个 bat文件启动 CoolCLI，我们管这个 bat文件叫 就叫做 LauncherBat


接下来 介绍 LauncherDir、CharacterRoot、CoolWorkspace 三个概念
三者互相解耦 没有必然关系


## LauncherDir 

LauncherDir 不是手动设置的
我们管 启动用的 bat 所在的 文件夹，叫做 LauncherDir

LauncherDir 文件夹中存在 .cool-system 文件夹，
其中包含 CoolCLI的 系统级别设置


```
LauncherDir\.cool-system   
│
├─ config.toml
├─ scope.toml
├─ command.toml
│
├─ logs
└─ state
```


## CharacterRoot 

规范的做法是 使用 cd命令 在 LauncherBat 中设置 CharacterRoot


CharacterRoot 指的是 CMD启动  CoolCLI时  先导航到的文件夹

举个例子，位于 Codex最下方显示的那个路径 就是我们定义的 CharacterRoot
![[file-20260612132052716.png]]


之所以起这个名字，是因为这个文件夹很可能包含 对于角色的 相关设计
而我使用 character 这个单词，没有使用 agent，是因为我希望更加拟人化。
而且 agent 这个名字都被叫混乱了，你都快搞不清楚 agent 它到底指的是 CLI 还是某一个 CLI 中的人格。


CharacterRoot  文件夹中 有 .cool文件夹，里面包含了 CharacterRoot 级别的 设置
以及相关缓存等


```
CharacterRoot\.cool   
│
├─ config.toml       普通项目配置
├─ scope.toml        视野配置
├─ command.toml      命令配置
│
├─ cache
├─ logs
└─ state
```


## CoolWorkspace

位于 CharacterRoot 中 .cool 的 config.toml    里设置

如果找不到 该设置，则 CoolWorkspace 默认等于 CharacterRoot

CoolWorkspace 是，CLI 工作聚焦的文件夹。
并且，CToolCommandRequest 工具默认基于的路径 是 CoolWorkspace
而不是 CharacterRoot


## 简称

**CoolSystemDir** 表示  .cool-system 文件夹路径
即 `LauncherDir\.cool-system`

**CoolDir** 表示 .cool 文件夹路径 
即 `CharacterRoot\.cool`

这么起简称，就是直接让文件夹和简称同名。

##
通过将三个 文件夹概念互相 解耦
好处在于 同一个工作区 可以用完全不同风格的 CoolCLI 进行操作
每个 CoolCLI 有自己的 CharacterRoot 级别的 设定
又能共享 System级别的设定 ，避免麻烦



