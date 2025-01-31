use reqwest;

async fn fetch(url: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;
    Ok(res.text().await?)
}

fn parse(content: String) -> Vec<String> {
    content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

async fn get(url: String) -> Option<Vec<String>> {
    match fetch(url).await {
        Ok(content) => Some(parse(content)),
        Err(err) => {
            println!("Error: {}", err);
            return None;
        }
    }
}

#[tokio::main]
pub async fn get_users() -> Vec<String> {
    let urls = [
        "https://raw.githubusercontent.com/danielmiessler/SecLists/refs/heads/master/Usernames/top-usernames-shortlist.txt",
    ];

    let mut lists = Vec::new();

    for url in urls {
        let lines = get(String::from(url)).await.unwrap();
        lists.extend(lines);
    }

    return lists;
}

#[tokio::main]
pub async fn get_passwords() -> Vec<String> {
    let urls = [
        "https://raw.githubusercontent.com/danielmiessler/SecLists/refs/heads/master/Passwords/Common-Credentials/10k-most-common.txt",
    ];

    let mut lists = Vec::new();

    for url in urls {
        let lines = get(String::from(url)).await.unwrap();
        lists.extend(lines);
    }

    return lists;
}
