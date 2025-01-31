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

#[tokio::main]
async fn get(url: String) -> Option<Vec<String>> {
    match fetch(url).await {
        Ok(content) => Some(parse(content)),
        Err(err) => {
            println!("Error: {}", err);
            return None;
        }
    }
}

pub fn get_users(_urls: Option<Vec<&str>>, strict: bool) -> Vec<String> {
    let mut urls = Vec::from([
        "https://raw.githubusercontent.com/danielmiessler/SecLists/refs/heads/master/Usernames/top-usernames-shortlist.txt",
    ]);

    let mut lists = Vec::new();

    if let Some(_urls) = _urls {
        if strict {
            urls.extend(_urls);
        }
    }

    for url in urls {
        let lines = get(String::from(url));
        if let Some(lines) = lines {
            lists.extend(lines);
        }
    }

    return lists;
}

pub fn get_passwords(_urls: Option<Vec<&str>>, strict: bool) -> Vec<String> {
    let mut urls = Vec::from([
        "https://raw.githubusercontent.com/danielmiessler/SecLists/refs/heads/master/Passwords/Common-Credentials/10k-most-common.txt",
    ]);

    let mut lists = Vec::new();

    if let Some(_urls) = _urls {
        if strict {
            urls.extend(_urls);
        }
    }

    for url in urls {
        let lines = get(String::from(url));
        if let Some(lines) = lines {
            lists.extend(lines);
        }
    }

    return lists;
}
