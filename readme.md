# unpack_option
for English ,see **[readmeEn](/readmeEn.md)**

一个轻量级Rust库，为Option类型提供扩展方法，便捷地将Option<T>转换为Result<T, OptionError>。
如果你是和`anyhow`一起使用的话，更推荐使用option.with_context(|| "option can not be null")?

## 功能特点
- 为Option<T>实现扩展 trait `OptionExt`
- 提供两种解包方法：`unpack()` 和 `unpack_for()`
- 自定义错误类型 `OptionError`，支持通用错误和参数名指定错误

## 安装
在`Cargo.toml`中添加依赖：
```toml
[dependencies]
unpack_option = "0.1.0"
```

## 使用示例
### 基本用法
```rust
use unpack_option::OptionExt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    // 成功解包Some值
    let value = some_value.unpack()?;
    println!("解包成功: {}", value);

    // 解包None值将返回错误
    match none_value.unpack() {
        Ok(_) => println!("这行不会执行"),
        Err(e) => println!("解包失败: {}", e), // 输出: "解包失败: param can not be null"
    }

    Ok(())
}
```

### 指定参数名
```rust
use unpack_option::OptionExt;

fn process_user(id: Option<u64>) -> Result<(), Box<dyn std::error::Error>> {
    // 为错误消息指定参数名
    let user_id = id.unpack_for("user_id")?;
    println!("处理用户ID: {}", user_id);
    Ok(())
}

fn main() {
    if let Err(e) = process_user(None) {
        println!("错误: {}", e); // 输出: "错误: user_id can not be null"
    }
}
```

## 错误类型
`OptionError` 枚举包含两个变体：
- `CanNotBeNull`: 通用错误，表示Option值为None
- `CanNotBeNullFor(&'static str)`: 包含参数名的错误，提供更具体的错误信息

## 许可证
本项目采用MIT许可证 - 详情参见[LICENSE](/license)文件