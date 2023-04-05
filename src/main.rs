// use tide_api::service::stateless;
// use tide_api::service::fileserver;
// use tide_api::service::state;
use tide_api::service::middleware;

/// async-std function 的入口函数
// #[async_std::main()]
// 使用 tokio 替换 async_std 
#[tokio::main()]
async fn main() -> tide::Result<()> {
    //
    // 无状态的静态服务
    // stateless::build_stateless_server().await
    //
    // 搭建静态文件服务器
    // fileserver::build_file_server().await
    //
    // 有数据状态的数据库操作
    // state::build_state_server().await
    //
    // 在请求链上使用自定义的中间件
    middleware::build_authorized_server().await

}
