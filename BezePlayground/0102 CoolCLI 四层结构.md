


## 第 1 层：SafeMode 

在 SafeMode 专文介绍


## 第2层：CLI的原生权限系统 例如 Codex的PermissionProfile


给原生权限系统 加入 Cool 开头的模式

例如 Codex的 Codex的PermissionProfile 我们加入 CoolReadWrite 模式

Cool 开头的 权限模式 只能使用 CTool 工具 （可能会增加极个别代码层经过审核放生的工具。）


我们并且要将原生的 权限系统 默认模式 设置为 CoolReadWrite 模式


### 注意
制作的时候，最早期 我们先加入 CoolReadWrite 然后不给他任何使用工具。
根据需要，一个一个添加 做到极致的安全

## 第三层 CToolScope

CToolScope 的设计哲学 不是要  直接限制 PermissionProfile 的视野范围
而是直接限制  限制 CTool 的视野范围 

SafeMode 下模型只能通过 CTool 工作，因此限制 CToolScope 就等于绝对限制模型可见范围

注：历史对话、告诉大模型身份的系统提示词、工具返回结果仍可能进入模型上下文。



#### CToolScopeBase

分为以下四种类型
None
CoolWorkspace
SelectedOnly
TheEyeOfProvidence


CToolScopeBase 默认设置为 None

#### 
关于视野的详细规则 请查阅 《CToolScope详细说明》 不在这里赘述



## 第4层 CTool


设计哲学

1/
CTool  要求在函数内用硬编码符合我们的视野规则。 在函数层面做到无法逾越视野规则。
2/ 要在 CoolCLI处于 CoolReadWrite权限时  清楚的知道自己当下能用的工具有哪些，该怎么用。
3/ CTool 都属于安全级别非常高的工具。 
除 CToolCommandRequest 和 CToolWebSearchRequest 之外
不能有 联网、下载和 编译可执行文件能力






## 启动 CoolCLI 时 要在最开始显示以下内容



SafeMode 状态
CharacterRoot  路径
PermissionProfile 状态
CoolWorkspace 路径

CToolScope 当前详细设置
	CToolScopeBase 状态
	配置文件中的具体设定

然后不要急着开启 codex
要确认一下 按Y再开启。
呃，如果按的是N 也不要急着退出
这时候可以使用 视野指令操作视野 配置
然后用/start 再次开始





在CLI窗口下方也持久显示（如果改造的原生CLI有内容 则位于原生内容的上一行）
SafeMode 开启增加一个小盾牌 CR： PP： CW：



视野操作指令 属于 CoolCLI 内置控制命令 不是CToolCommandRequest 可以申请的
但假如出现能修改视野的 指令 对于CToolCommandRequest 属于Block级别


注：这里再陈述以下设计哲学
CLI 的安全是靠 SafeMode 确定的。 
而 CToolScope  只能确定大模型在使用 CTool 的时候的安全边界
CToolScope  并不是用来影响 CLI 的。 因而位于 CLI 启动之后、大模型可操作之前，来补救视野设定是很合适的。




