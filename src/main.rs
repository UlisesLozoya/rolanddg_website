use tide::{Body, Response, Request};

async fn handle_home(_: Request<()>) -> tide::Result {
    let html_content = async_std::fs::read_to_string("html/homepage.html").await?;
    Ok(Response::builder(200)
        .body(Body::from_string(html_content))
        .header("Content-Type", "text/html")
        .build())
}

async fn handle_category(req: Request<()>) -> tide::Result {
    let category = req.param("category")?;
    let file_path = format!("html/{}.html", category);

    match async_std::fs::read_to_string(&file_path).await {
        Ok(html_content) => Ok(Response::builder(200)
            .body(Body::from_string(html_content))
            .header("Content-Type", "text/html")
            .build()),
        Err(err) => {
            eprintln!("Error reading {}: {}", &file_path, err);
            Ok(Response::builder(500).body(Body::empty()).build())
        }
    }
}

async fn handle_welcome(_: Request<()>) -> tide::Result {
    let html_content = async_std::fs::read_to_string("html/welcome_page.html").await?;
    Ok(Response::builder(200)
        .body(Body::from_string(html_content))
        .header("Content-Type", "text/html")
        .build())
}

async fn handle_video(_: Request<()>) -> tide::Result {
    
    let video_content = async_std::fs::read_to_string("html/video.html").await?;
    Ok(Response::builder(200)
        .body(Body::from_string(video_content))
        .header("Content-Type", "text/html")
        .build())
}

fn main() {
    async_std::task::block_on(async {
        let mut app = tide::new();
        app.at("/static").serve_dir("static").unwrap();
        app.at("/").get(handle_home);
        app.at("/:category").get(handle_category);
        app.at("/:category/welcome_page").get(handle_welcome);
        app.at("/:category/video").get(handle_video);

        let addr = "127.0.0.1:8080";
        println!("Server running on http://{}", addr);

        app.listen(addr).await.expect("Failed to start server");
    });
}
