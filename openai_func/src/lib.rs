use gandiva_rust_udf_macro::udf;

use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;
use openai_api_rs::v1::common::GPT3_5_TURBO;
use chat_completion::MessageRole::{user, system};

#[udf]
fn askai(prompt: &str) -> String {
    let result = _call_openai(&prompt, "");
    result
}

#[udf]
fn ai_extract(data: &str) -> String {
    let result = _call_openai(
        data,
        "Extract the entities from the given data as key value pairs. \
                              You are an assistant that only speaks JSON. \
                              Use lower case for JSON key names.\
                              Do not write normal text.",
    );
    result
}

#[udf]
fn ai_func(user_content: &str, system_content: &str) -> String {
    _call_openai(user_content, system_content)
}

fn _chat_message(role: chat_completion::MessageRole, content: &str) -> chat_completion::ChatCompletionMessage {
    chat_completion::ChatCompletionMessage {
        role,
        content: chat_completion::Content::Text(String::from(content)),
        name: None,
    }
}

fn _call_openai(user_content: &str, system_content: &str) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    // OPENAI_API_BASE env var can be set to change the base URL
    // by default, it is `https://api.openai.com/v1`
    let client = Client::new(api_key);
    // if system_content is not empty, construct a system content message
    let mut messages = Vec::new();

    if !system_content.is_empty() {
        messages.push(_chat_message(system, system_content));
    }
    messages.push(_chat_message(user, user_content));

    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        messages,
    );
    let result = client.chat_completion(req);
    match result {
        Ok(result) => {
            let message = result.choices[0].message.content.clone();
            message.unwrap()
        }
        Err(e) => {
            format!("err: {}", e)
        }
    }
}

#[cfg(test)]
mod openai_tests {
    use super::*;

    // #[test]
    fn test_askai() {
        let result = askai("Meaning of this number? '13910998888'");
        assert!(result.len() > 0);
        assert!(
            !result.contains("err"),
            "Unexpected 'err' in result: {}",
            result
        );
        println!("{}", result)
    }

    // #[test]
    fn test_ai_extract() {
        let result = ai_extract("Apple Vision Pro is announced on 2021-10-18.");
        assert!(result.len() > 0);
        assert!(
            !result.contains("err"),
            "Unexpected 'err' in result: {}",
            result
        );
        println!("{}", result)
    }
}
