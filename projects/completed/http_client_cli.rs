use std::{env, time::Duration};

#[tokio::main]
async fn main() {
    match run(&env::args().skip(1).collect::<Vec<_>>()).await {
        Ok(output) => print!("{output}"),
        Err(error) => eprintln!("Error: {error}"),
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Options {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    form: Vec<(String, String)>,
    headers_only: bool,
    json: bool,
    timeout_seconds: u64,
}

fn parse_options(arguments: &[String]) -> Result<Options, String> {
    let url = arguments.first().ok_or_else(|| "usage: http_client_cli URL [--method METHOD] [--header NAME:VALUE] [--data BODY] [--form KEY=VALUE] [--headers-only] [--json] [--timeout SECONDS]".to_string())?.clone();
    let mut options = Options {
        method: "GET".to_string(),
        url,
        timeout_seconds: 30,
        ..Options::default()
    };
    let mut index = 1;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--method" | "--header" | "--data" | "--form" | "--timeout" => {
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| format!("{} needs a value", arguments[index]))?
                    .clone();
                match arguments[index].as_str() {
                    "--method" => options.method = value.to_uppercase(),
                    "--header" => {
                        let (name, value) = value
                            .split_once(':')
                            .ok_or_else(|| "header must use NAME:VALUE".to_string())?;
                        options
                            .headers
                            .push((name.trim().to_string(), value.trim().to_string()));
                    }
                    "--data" => options.body = Some(value),
                    "--form" => {
                        let (name, value) = value
                            .split_once('=')
                            .ok_or_else(|| "form field must use KEY=VALUE".to_string())?;
                        options.form.push((name.to_string(), value.to_string()));
                    }
                    "--timeout" => {
                        options.timeout_seconds = value
                            .parse()
                            .map_err(|_| "timeout must be an integer".to_string())?
                    }
                    _ => unreachable!(),
                }
                index += 1;
            }
            "--headers-only" => options.headers_only = true,
            "--json" => options.json = true,
            value => return Err(format!("unknown option '{value}'")),
        }
        index += 1;
    }
    options
        .method
        .parse::<reqwest::Method>()
        .map_err(|_| format!("invalid HTTP method '{}'", options.method))?;
    Ok(options)
}

async fn run(arguments: &[String]) -> Result<String, String> {
    let options = parse_options(arguments)?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(options.timeout_seconds))
        .build()
        .map_err(|error| format!("failed to build client: {error}"))?;
    let method = options
        .method
        .parse::<reqwest::Method>()
        .map_err(|error| error.to_string())?;
    let mut request = client.request(method, &options.url);
    for (name, value) in &options.headers {
        request = request.header(name, value);
    }
    if !options.form.is_empty() {
        let form = options
            .form
            .iter()
            .map(|(name, value)| {
                format!("{}={}", encode_form_value(name), encode_form_value(value))
            })
            .collect::<Vec<_>>()
            .join("&");
        request = request
            .header("content-type", "application/x-www-form-urlencoded")
            .body(form);
    } else if let Some(body) = &options.body {
        request = request.body(body.clone());
    }
    let response = request
        .send()
        .await
        .map_err(|error| format!("request failed: {error}"))?;
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
        .text()
        .await
        .map_err(|error| format!("failed to read response: {error}"))?;
    if !status.is_success() {
        return Err(format_response(status, &headers, &body));
    }
    if options.headers_only {
        Ok(format_headers(status, &headers))
    } else if options.json {
        serde_json::from_str::<serde_json::Value>(&body)
            .map_err(|error| format!("response is not valid JSON: {error}"))?;
        Ok(body)
    } else {
        Ok(format_response(status, &headers, &body))
    }
}

fn encode_form_value(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                char::from(byte).to_string()
            }
            byte => format!("%{byte:02X}"),
        })
        .collect()
}

fn format_headers(status: reqwest::StatusCode, headers: &reqwest::header::HeaderMap) -> String {
    let mut output = format!("HTTP {}\n", status);
    for (name, value) in headers {
        output.push_str(&format!(
            "{}: {}\n",
            name,
            value.to_str().unwrap_or("<binary>")
        ));
    }
    output
}

fn format_response(
    status: reqwest::StatusCode,
    headers: &reqwest::header::HeaderMap,
    body: &str,
) -> String {
    format!(
        "{}{}\n{}",
        format_headers(status, headers),
        if body.is_empty() { "" } else { "\n" },
        body
    )
}

#[cfg(test)]
mod tests {
    use super::{Options, format_response, parse_options};
    use reqwest::{StatusCode, header::HeaderMap};

    #[test]
    fn parses_request_flags_and_repeated_forms() {
        let args = vec![
            "http://example.test".into(),
            "--method".into(),
            "post".into(),
            "--header".into(),
            "X-Test: yes".into(),
            "--form".into(),
            "a=1".into(),
            "--form".into(),
            "b=2".into(),
        ];
        let options = parse_options(&args).unwrap();
        assert_eq!(options.method, "POST");
        assert_eq!(options.headers, vec![("X-Test".into(), "yes".into())]);
        assert_eq!(options.form.len(), 2);
    }

    #[test]
    fn rejects_invalid_flags() {
        assert!(parse_options(&["url".into(), "--header".into(), "broken".into()]).is_err());
        assert!(parse_options(&["url".into(), "--method".into(), "???".into()]).is_err());
    }

    #[test]
    fn formats_status_headers_and_body() {
        let headers = HeaderMap::new();
        let output = format_response(StatusCode::OK, &headers, "hello");
        assert!(output.contains("HTTP 200 OK"));
        assert!(output.ends_with("\nhello"));
        let _ = Options::default();
    }
}
