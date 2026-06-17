
## 名词设定


管采用 Cool 架构的 CLI， 不管是深度改造 的开源CLI，还是纯粹从零开始开发的 ，都叫做 **CoolCLI**

被 Cool架构深度改造过的 叫 **CoolCodex**

用 Cool架构 从零创造的第一个 CLI叫做  **CodeElemental**


我们通过一个 bat文件启动 CoolCLI，我们管这个 bat文件叫 就叫做 **LauncherBat**


接下来 介绍 


## 三个路径概念

LauncherDir、CoolCharacterRoot、CoolWorkspace 三个概念
三者互相解耦 没有必然关系

通过将三个 文件夹概念互相 解耦 ，可以让 同一个工作区 有不同风格的Character进行工作
同时 LauncherBat 都集中在 LauncherDir 可以共享 系统级别设定 避免麻烦

### LauncherDir 

LauncherDir 非通过变量设置 LauncherBat 所在的文件夹即为 LauncherDir
LauncherDir 文件夹中存在 .cool-system 文件夹，里面保存 CoolCLI的 系统级别设置


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


### CoolCharacterRoot 

在正式启动CLI的 exe文件之前，会先导航到 CoolCharacterRoot 文件夹再启动。
举个例子 CoolCharacterRoot路径，也就是位于 Codex最下方显示的那个路径
（这样设定有一个好处，如果改造的CLI如果有残留，会产生文件，也会产生在专属文件夹里，不会搞乱环境）
![[file-20260612132052716.png]]

CharacterRoot  文件夹中 有 .cool文件夹，里面包含了 CharacterRoot 级别的 设置
以及相关缓存等


```
CoolCharacterRoot\.cool   
│
├─ config.toml       普通项目配置
├─ scope.toml        视野配置
├─ command.toml      命令配置
│
├─ cache
├─ logs
└─ state
```


#### 命名哲学
之所以用 Character这个单词，是因为Agent这个名字被叫混乱了。 快分不清到底是CLI还是某个具体人格。
并且Character也更加拟人化


### CoolWorkspace
CoolWorkspace 是一个 CoolCLI的 当前 Character主要工作区

CoolWorkspace 是 CToolCommandRequest 工具默认基于的路径

注意：
CToolCommandRequest 工具默认基于的路径是 CoolWorkspace
而非 CoolCharacterRoot

## 路径简称补充

###
设置文件中如果出现`.`  指代的就是 CoolCharacterRoot 路径
所有的相对路径，都是以 CoolCharacterRoot 为起点

### LaunchDir
CMD 召唤 LaucherBat 时 所位于的路径

### CoolSystemDir
表示 `LauncherDir\.cool-system`

命名哲学：文件夹和简称同名

### CoolDir
表示 `CoolCharacterRoot\.cool`


### CoolCLIExePath
表示 CoolCLI的 exe文件 的文件绝对路径



