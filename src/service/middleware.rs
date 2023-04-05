use std::future::Future;
use std::pin::Pin;
use tide::{Next, Request, Response, Result};

/// 自定义中间件，用于检验请求钟是否含有认证信息
///
fn check_req<'a>(
    mut req: Request<()>,
    next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        if req.header("Authorization").is_none() {
            let mut res = Response::new(401);
            res.set_body("Forbidden");
            return Ok(res);
        }
        // 如果需要继续向下传递对应的认证信息
        // 这时候我们需要 req 被 mut 修饰
        req.set_ext(req.header("Authorization").unwrap().to_string());
        // 让请求链继续向下执行
        Ok(next.run(req).await)
    })
}

pub async fn build_authorized_server() -> Result<()> {
    femme::start();
    let mut app = tide::new();
    // 使用自定义的中间件
    app.with(check_req);

    // 监听路由
    app.at("/admin").get(|req: Request<()>| async move {
        // 这里可以接收到传递的认证信息
        println!("{}", req.ext::<String>().unwrap());
        Ok("Hello, admin")
    });
    app.listen("0.0.0.0:8888").await?;

    Ok(())
}
