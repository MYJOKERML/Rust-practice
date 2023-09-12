use tokio::runtime::Runtime;

async fn get_html(url: &str) -> Result<reqwest::Response, reqwest::Error>{
    let resp = reqwest::get(url).await?;
    Ok(resp)
}

fn main() {
    let url = "https://www.toutiao.com";
    let fut = get_html(url);
    let rt = Runtime::new().unwrap();
    let resp = rt.block_on(fut).unwrap();
    println!("response: {:?}, {}", resp.version(), resp.status());

    // for header in resp.headers().iter() {
    //     println!("{:?}: {:?}", header.0, header.1);
    // }

    
    let body = resp.text();
    let body = rt.block_on(body).unwrap();
    println!("body: {}", body);
}
