use reqwest::Client;
use std::{
    collections::{HashSet, VecDeque},
    env,
    sync::Arc,
};
use tokio::sync::Semaphore;
use url::Url;

#[tokio::main]
async fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match crawl_command(&args).await {
        Ok(urls) => {
            for url in urls {
                println!("{url}");
            }
        }
        Err(error) => eprintln!("Error: {error}"),
    }
}

async fn crawl_command(args: &[String]) -> Result<Vec<Url>, String> {
    let start = args
        .first()
        .ok_or_else(|| "usage: crawler URL [--depth N] [--concurrency N]".to_string())?
        .parse::<Url>()
        .map_err(|error| format!("invalid URL: {error}"))?;
    let mut depth = 1usize;
    let mut concurrency = 4usize;
    let mut index = 1;
    while index < args.len() {
        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("{} needs a value", args[index]))?;
        match args[index].as_str() {
            "--depth" => {
                depth = value
                    .parse()
                    .map_err(|_| "depth must be an integer".to_string())?
            }
            "--concurrency" => {
                concurrency = value
                    .parse()
                    .map_err(|_| "concurrency must be an integer".to_string())?
            }
            option => return Err(format!("unknown option '{option}'")),
        }
        if concurrency == 0 {
            return Err("concurrency must be positive".to_string());
        }
        index += 2;
    }
    crawl(start, depth, concurrency).await
}

async fn crawl(start: Url, max_depth: usize, concurrency: usize) -> Result<Vec<Url>, String> {
    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(concurrency));
    let mut visited = HashSet::from([start.clone()]);
    let mut frontier = VecDeque::from([(start, 0usize)]);
    let mut output = Vec::new();
    while !frontier.is_empty() {
        let mut tasks = Vec::new();
        while let Some((url, level)) = frontier.pop_front() {
            output.push(url.clone());
            if level >= max_depth {
                continue;
            }
            let client = client.clone();
            let semaphore = semaphore.clone();
            tasks.push(tokio::spawn(async move {
                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .map_err(|_| "semaphore closed".to_string())?;
                let body = client
                    .get(url.clone())
                    .send()
                    .await
                    .map_err(|error| error.to_string())?
                    .error_for_status()
                    .map_err(|error| error.to_string())?
                    .text()
                    .await
                    .map_err(|error| error.to_string())?;
                Ok::<_, String>((url, body, level + 1))
            }));
        }
        for task in tasks {
            let (base, body, level) = task.await.map_err(|error| error.to_string())??;
            for link in extract_links(&body, &base)? {
                if visited.insert(link.clone()) {
                    frontier.push_back((link, level));
                }
            }
        }
    }
    Ok(output)
}

fn extract_links(body: &str, base: &Url) -> Result<Vec<Url>, String> {
    let mut links = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("href=") {
        rest = &rest[start + 5..];
        let quote = rest
            .chars()
            .next()
            .ok_or_else(|| "href has no value".to_string())?;
        if quote != '"' && quote != '\'' {
            continue;
        }
        rest = &rest[1..];
        let end = rest
            .find(quote)
            .ok_or_else(|| "unterminated href".to_string())?;
        let raw = &rest[..end];
        rest = &rest[end + 1..];
        let link = base
            .join(raw)
            .map_err(|error| format!("invalid link '{raw}': {error}"))?;
        if link.scheme() == base.scheme()
            && link.domain() == base.domain()
            && !links.contains(&link)
        {
            links.push(link);
        }
    }
    Ok(links)
}

#[cfg(test)]
mod tests {
    use super::extract_links;
    use url::Url;

    #[test]
    fn resolves_relative_and_filters_domains() {
        let base = Url::parse("https://example.test/docs/start").unwrap();
        let links = extract_links(r#"<a href="/one">one</a><a href="next">next</a><a href="https://other.test/x">off</a>"#, &base).unwrap();
        assert_eq!(
            links,
            vec![
                Url::parse("https://example.test/one").unwrap(),
                Url::parse("https://example.test/docs/next").unwrap()
            ]
        );
    }

    #[test]
    fn handles_single_quotes_and_empty_pages() {
        let base = Url::parse("https://example.test/").unwrap();
        assert_eq!(
            extract_links("<p>none</p>", &base).unwrap(),
            Vec::<Url>::new()
        );
        assert_eq!(extract_links("<a href='/x'>x</a>", &base).unwrap().len(), 1);
    }
}
