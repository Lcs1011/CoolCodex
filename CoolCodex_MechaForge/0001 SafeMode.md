
SafeMode 是针对 **CLI本身的** 严格限制


safe-mode 设计原则 **Above All** 
启动main前 默认值为 On  ，staitc 只读变量

--safe-mode 默认值为 On
并且进入 main后 第一时间解析 对 --safe-mode 的设置
解析完成后 变为全局只读状态

//Rust 进入main之前 没有 C++自动跑一堆全局对象构造函数，所有参数会保持默认值进入Rust



模型 API 推理所必须的联网请求、账号登录所必需的网络请求除外
其余网络请求必须被 SafeMode 审核或阻断。

对于 plugin 和 skill 的管理，另外文章。