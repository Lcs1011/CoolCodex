
## 文件结构


### 标准路径

启动器位置
`C:\Arsenal\CoolAI\Launchers\cbot.bat` 

配置位置
`C:\Arsenal\CoolAI\Launchers\.cool-system\coolbot-config.toml`

写rust脚本的位置
`C:\Arsenal\CoolAI\CoolBot` 

```
C:\Arsenal\CoolAI\
│
├── Launchers\
│   ├── cbot.bat  启动器位置
│   └── .cool-system\
│       └── coolbot-config.toml  Coolbot设置位置
│
└── CoolBot\
    ├── .git\
    ├── .gitignore
    ├── Cargo.toml
    └── src\
        └── main.rs   Rust主程序入口
```

##### KP
之前写 Obsidian 插件的时候，核心文件叫 main.ts 。和main.rs一字之差
巧的是它们都位于 src 文件夹下。src 代表的是 source 的意思。



# 包含的各个功能


### 指令


唤醒指令为
```
/CoolBot 
```
或
```
/CBot 
```

`/CBot c` 并回车时
执行剪切板的内容 c 代表 clipboard


`/CBot d 并回车时

d代表 default
没有默认路径 产生输入框 让当下设置
设置内容会保存到 toml
任何情况记得 默认路径没有 或者全是空格 就是没设置
当我输入 `/CBot ` 并回车时
如果 `/CBot ` 有md文件的绝对路径，就执行该 md的内容
如果没有文件的绝对路径 并且配置了默认路径 就执行默认路径
如果都没有 弹出提示 是否执行剪切板里的内容


/CBot status 显示当前状态
执行输出md路径


### 需要在 coolbot-config.toml 设置的内容

#### 执行相关
默认执行路径
报告输出路径

#### git 相关
需要git push的路径
工程路径
笔记路径 空就是有

#### telegram 相关
telegram token
主人 id

单括号
`[behavior]`
**双括号 `[[ ]]`**。

### 达到的效果

第一行必须是 OperationID <起的名字>
并且

链接telegrame

唤醒指令为
```
/CoolBot 
```
或
```
/CBot 
```

当我输入 `/CBot ` 并回车时
如果 `/CBot ` 有md文件的绝对路径，就执行该 md的内容
如果没有文件的绝对路径 并且配置了默认路径 就执行默认路径
如果都没有 弹出提示 是否执行剪切板里的内容


/CBot status 显示当前状态
执行输出md路径


set 执行md
set 输出md
清空设置

.cool文件夹内
coolbot-config.toml

提示

第一行必须是 OperationID <起的名字>否则的话，就不能执行
然后脚本会先完整检查一下执行内容 其中有不可识别的部分，就不执行


``` toml
# `.cool/coolbot-config.toml` 示范配置

[paths]
# 1. 默认执行路径：当你在控制台只敲了 `cbot` 而没有给绝对路径时，去哪里找默认脚本
default_script = "D:\\Projects\\Eidolon\\Scripts\\DefaultOperation.md"

# 2. 报告输出路径：CoolOperationReport.md 应该保存在哪里？
# 如果留空或者不写，默认就保存在脚本所在的同级目录
report_output_dir = "D:\\Projects\\Eidolon\\Logs"

[behavior]
# 遇到无法识别的指令时是否直接报错退出 (对应你的前端语义校验)
strict_mode = true

# 从剪贴板读取前，是否必须弹窗询问 (防止误执行危险代码)
ask_before_clipboard = true
```


