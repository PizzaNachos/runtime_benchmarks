#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:3000/sleep/100";
    let times = 1;
    let threads = 1000;

    let mut tasks = vec![];
    for _ in 0..threads{
        let handle = tokio::spawn(async move{
            let mut time = std::time::Duration::new(0,0);
            for _ in 0..times{
                let Ok(t) = time_response(url).await else {
                    continue;
                };
                // println!("{:?}", t);
                assert!(t.1 == "100");
                time += t.0;
            }
        
            println!("{:?}",time / times);



        });
        tasks.push(handle)
    }

    for h in tasks{
        h.await;
    }

    Ok(())
}

async fn time_response(url:&str) -> Result<(std::time::Duration,String),Box<dyn std::error::Error>> {
    let c = reqwest::Client::new().get(url);
    let start = std::time::Instant::now();
    let resp: reqwest::Response = c.send().await?;
    let stop = std::time::Instant::now();

    let body: String = resp.text().await?;
    // println!("{}",body);
    return Ok((stop - start, body));
}