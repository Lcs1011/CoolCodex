## 配置文件概述

bat 中 设置

CoolSystem级 配置位于： 该路径简称为 CoolSystemDir

```text
LauncherDir\.cool-system\
```

CoolCharacter 配置位于： 该路径简称为 CoolDir

```text
CharacterRoot\.cool\
```



### Launcher bat中一共要设置 6样东西
1/ CoolCharacterRoot 路径
2/ CoolWorkSpace 是否跟随 启动窗口
3/ CoolWorkSpace 默认路径
4/ CToolScopeBase枚举 默认值

5/ SafeMode 开关
6/ PermissionProfile枚举 默认值

###
CoolSystemDir 和 CoolDir
各有
config.toml  //默认设置 暂时为空
scope.toml // 视野设置
command.toml //命令设置



## bat

```bat
@echo off
setlocal

rem ============================================================
rem CTool Settings
rem ============================================================

rem Enter path
set "COOL_CHARACTER_ROOT=C:\Arsenal\CoolAI\CoolBot"

rem on/off
set "COOL_WORKSPACE_FOLLOW_LAUNCH_DIR=off"

rem Enter path
rem Relative path starts from COOL_CHARACTER_ROOT
set "COOL_WORKSPACE=C:\Arsenal\CoolAI\CoolBot"

rem none/selected-only/cool-workspace/the-eye-of-providence
set "CTOOL_SCOPE_BASE=cool-workspace"

rem green/yellow/red/block/block-all
set "CTOOL_COMMAND_POLICY=yellow"


rem ============================================================
rem Core Safety Settings
rem Recommended: do not change.
rem ============================================================

rem on/off
set "COOL_SAFE_MODE=on"

rem cool-read-write/read-only/workspace-write/full-access
set "COOL_PERMISSION_PROFILE=cool-read-write"

rem ============================================================
rem Internal Launch Settings
rem Do not edit.
rem ============================================================

rem Launch directory
set "LAUNCH_DIR=%CD%"

rem Launcher directory
set "LAUNCHER_DIR=%~dp0"

rem Codex executable path
set "CODEX_EXE=C:\Arsenal\CoolCurrent\codex.exe"

rem Cool system directory
set "COOL_SYSTEM_DIR=%LAUNCHER_DIR%.cool-system"

```

注：
SAFE_MODE和 PERMISSION_PROFILE 太通用 前面加 COOL_为了防止在CMD中意外撞名

## Scope设置介绍


### CToolScopeBase 枚举
#### none 
无视野，纯粹无任何视野 toml文件也无法添加视野。  启动时，找不到CToolScopeBase设定，则将其设置为 none
#### selected-only 
无基础视野仅凭设定
#### cool-workspace  
CoolWorkspace文件夹为基础视野
#### the-eye-of-providence 
全局视野//暂不开放，不制作无视和 cool-workspace 相同处理


### 原则

要满足西面三个情况
1/ 系统封杀的文件夹 角色的文件许可也绝对不能开开
2/ 系统开放的 角色要能封掉 （解决办法就是系统j 尽量不要默认别开任何视野，否则角色级只能动用特权封）
3/ 同一个级别的文件 开放 文件夹关闭 要让该文件夹可访问

原则等级排序
1/ 特权 大于 非特权    //轻易不要填写特权，特权要留着关键时候用
2/ 系统 大于 角色  //系统尽量不要给 ReadWrite 或 ReadOnly
3/ 文件 大于 文件夹
4/ 禁止 大于 开放
5/ 都大于  CToolScopeBase  //None除外



```
CToolScopeBase:none>

Privileged 内部如下不赘述 整体 > 

System.Files.Hidden > System.Files.ReadOnly > System.Files.ReadWrite >
System.Folders.Hidden > System.Folders.ReadOnly > System.Folders.ReadWrite >
Character.Files.Hidden > Character.Files.ReadOnly > Character.Files.ReadWrite >
Character.Folders.Hidden > Character.Folders.ReadOnly > Character.Folders.ReadWrite >
CToolScopeBase:非none
```



### Scope.toml

#### 空表

``` toml
[privileged_files]

readwrite = []
readonly = []
hidden = []


[privileged_folders]

readwrite = []
readonly = []
hidden = []


[files]

readwrite = []
readonly = []
hidden = []


[folders]

readwrite = []
readonly = []
hidden = []

```

#### 系统级
``` toml
[privileged_files]

readwrite = []
readonly = []
hidden = []


[privileged_folders]

readwrite = []
readonly = []
hidden = []


[files]

readwrite = []
readonly = []
hidden = []


[folders]

readwrite = []
readonly = []
hidden = [
	".cool-system",  
	".cool",
	".git",
]

```


#### 角色级
默认为空


## Command介绍

CToolCommandPolicy 枚举
表示如果找不到设定的默认级别，blockall 阻断一切
找不到设定则 设置为blockall
Green/Yellow/Red/Blocked/AllBlocked

Treat as Green


特权 大于 非特权  //轻易不要填写特权，特权要留着关键时候用
禁止 大于 开放
都大于 默认 //blockall除外

做法是 先合并 相同性质的集合，然后再排序。

```
Privileged 内部如下不赘述 整体大于>
System.Blocked + Character.Blocked > System.Red + Character.Red > System.Yellow + Character.Yellow > System.Green + Character.Green
```

### 空表演示

``` toml
[ctool_command_privileged]

blocked_prefixes = []
blocked_contains = []

red_prefixes = []
red_contains = []

yellow_prefixes = []

green_exact_commands = []
green_prefixes = []


[ctool_command]

enabled = true

blocked_prefixes = []
blocked_contains = []

red_prefixes = []
red_contains = []

yellow_prefixes = []

green_exact_commands = []
green_prefixes = []
```

### 系统级默认





### 角色级默认空

