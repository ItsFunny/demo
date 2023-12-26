// // 文件名：lib.rs
//
// use tokio::time::{sleep, Duration};
//
// #[no_mangle]
// pub extern "C" fn run_async_task() {
//     // 启动异步任务
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         async_task().await;
//     });
// }
//
// async fn async_task() {
//     // 异步任务示例，这里简单地等待一秒钟
//     println!("Async task started!");
//     sleep(Duration::from_secs(1)).await;
//     println!("Async task completed!");
// }

#[repr(C)]
pub struct MyStruct {
    pub field1: i32,
    pub field2: f64,
}

// Rust 中的函数，返回一个结构体
// #[no_mangle]
// pub extern "C" fn get_struct() -> MyStruct {
//     MyStruct {
//         field1: 42,
//         field2: 3.14,
//     }
// }

#[no_mangle]
pub extern "C" fn get_aa() -> i32 {
    1i32
}



