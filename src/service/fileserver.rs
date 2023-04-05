use tide::Result;

/// --------------------------------------------------------
/// 构建一个静态的文件服务器 file serving
/// --------------------------------------------------------
pub async fn build_file_server() -> Result<()> {
    femme::start();
    let mut app = tide::new();
    // 代理文件
    app.at("/hello").serve_file("./hello.txt")?;
    // 代理目录
    app.at("/www").serve_dir("./file")?;

    // 监听服务
    app.listen("0.0.0.0:8888").await?;

    Ok(())
}
