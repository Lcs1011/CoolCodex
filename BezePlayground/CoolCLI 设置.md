
bat要设定

characterRoot
Coolworkspace 是不是动态
Coolworkspace 的静态地址



可以在 main函数启动前 加载bat中的设定
但无法在main函数启动前 加载 toml 但是可以采取 ，让 SafeMode 默认为保守值，然后第一个加载。


BAT 虽然加载了，但是并不影响那个 Rust 里面的变量。影响Rust 里面的变量的话，还是得 main 函数运行后 加载才行

