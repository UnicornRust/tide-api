use crate::models::{model::Wizard, param::NameParams};
use tide::http::cookies::Cookie;
use tide::{Request, Response, Result};

pub async fn build_stateless_server() -> Result<()> {
    femme::start();

    let mut app = tide::new();

    // 日志追踪记录
    app.with(tide::log::LogMiddleware::new());

    // first index slash
    app.at("/").get(|_| async { Ok("Hello world!!") });
    // split handle function
    app.at("/home").get(handle_home);
    // 参数 name=alex&age=18 这样的参数对解析
    app.at("/param_pair").get(handle_param_pair);
    // 参数解析成为一个结构体
    app.at("/param_struct").get(handle_param_struct);
    // 解析路径参数 `:name`
    app.at("/param_path/:name").get(handle_path_param);
    //  post  获取请求参数并发序列化到结构体中
    app.at("/post_json").post(post_json);
    //  请求获取 json 数据
    app.at("/return_json").get(return_json);

    // --------------------------------------------------------
    // 接口嵌套 nest
    // 内部可以是很多接口的组合，他们都有公共的前缀
    app.at("/nest_api").nest({
        let mut api = tide::new();

        api.at("/")
            .post(|_req: Request<()>| async move { Ok("create") });

        api.at("/:id")
            .get(|_req: Request<()>| async move { Ok("Read!") });
        // 最后返回这个 内嵌的 api 对象
        api
    });

    // --------------------------------------------------------
    // 返回响应的新建
    app.at("/new_resp").post(new_response);
    // 构建这模式响应
    app.at("/build_resp").post(build_response);

    // --------------------------------------------------------
    // Cookie 操作
    app.at("/set_cookie").get(set_cookie);
    // --------------------------------------------------------

    // 监听端口
    app.listen("0.0.0.0:8888").await?;

    Ok(())
}

// 定义对应的路由的处理函数
async fn handle_home(_req: tide::Request<()>) -> tide::Result<String> {
    Ok("Hello home".to_string())
}

// 处理请求参数
// name=alex&age=18
async fn handle_param_pair(req: tide::Request<()>) -> tide::Result<String> {
    let name = req
        .url()
        .query_pairs()
        .find(|(k, _)| k == "name")
        .map(|(_, v)| v);

    Ok(format!("Hello, {}", name.unwrap_or("world".into())))
}

//
async fn handle_param_struct(req: tide::Request<()>) -> tide::Result<String> {
    let params: NameParams = req.query()?;
    Ok(format!("Hello {}!", params.name))
}

async fn handle_path_param(req: tide::Request<()>) -> tide::Result<String> {
    let name = req.param("name").unwrap_or("world");
    Ok(format!("Hello, {}", name))
}

// 使用到 serde_json 方式进行序列化
// 读取 json 格式的数据然后进行操作
// 由于这里需要读取 req body 里面的数据，所以是 mut 修饰
async fn post_json(mut req: tide::Request<()>) -> tide::Result<String> {
    let wizard: Wizard = req.body_json().await?;
    Ok(format!("{} is level {}!", wizard.name, wizard.level))
}

async fn return_json(_req: tide::Request<()>) -> tide::Result<tide::Body> {
    let wizards = vec![
        Wizard {
            name: "Gandalf".to_string(),
            level: 100,
        },
        Wizard {
            name: "Merlin".to_string(),
            level: 64,
        },
    ];
    Ok(tide::Body::from_json(&wizards)?)
}

async fn new_response(_req: Request<()>) -> tide::Result {
    // 可以写响应码的数字表示响应状态
    // let mut res = Response::new(201);
    // 可以使用内置的状态枚举来构建响应的状态码
    let mut res = Response::new(tide::StatusCode::Created);
    // 设置响应的数据
    res.set_body("New Response");
    Ok(res)
}

async fn build_response(_req: Request<()>) -> tide::Result {
    // 使用构建模式来构建响应
    let res = Response::builder(tide::StatusCode::Created)
        .body("Build Response")
        .build();
    Ok(res)
}

async fn set_cookie(req: Request<()>) -> Result {
    let name = req.cookie("name").unwrap();
    let mut res = Response::new(200);
    res.set_body(format!("Hello, {}!", name.value()));

    // 设置响应中的 cookie
    res.insert_cookie(Cookie::new("app", "Tide"));
    // 删除响应中的 cookie
    res.remove_cookie(Cookie::new("name", "foo"));

    Ok(res)
}
