use gandiva_rust_udf_macro::udf;

use openai_api_rs::v1::api::Client;
use std::env;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};

#[udf]
fn askai(prompt: &str) -> String {
    let result = _call_openai(&prompt, "");
    result
}

#[udf]
fn ai_extract(data: &str) -> String {
    let result = _call_openai(data,
                              "Extract the entities from the given data as key value pairs. \
                              You are an assistant that only speaks JSON. \
                              Use lower case for JSON key names.\
                              Do not write normal text.");
    result
}

#[udf]
fn ai_func(user_content: &str, system_content: &str) -> String {
    _call_openai(user_content, system_content)
}

fn _call_openai(user_content: &str, system_content: &str) -> String {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let client = Client::new(api_key);
    // if system_content is not empty, construct a system content message
    let mut messages = Vec::new();

    if !system_content.is_empty() {
        messages.push(chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::system,
            content: String::from(system_content),
            name: None,
            function_call: None,
        });
    }
    messages.push(chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: String::from(user_content),
        name: None,
        function_call: None,
    });

    let req = ChatCompletionRequest {
        model: chat_completion::GPT3_5_TURBO.to_string(),
        messages,
        functions: None,
        function_call: None,
        temperature: None,
        top_p: None,
        n: None,
        stream: None,
        stop: None,
        max_tokens: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };
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
mod tests {
    use super::*;

    // #[test]
    fn test_askai() {
        let result = askai("Meaning of this number? '13910998888'");
        assert!(result.len() > 0);
        assert!(!result.contains("err"), "Unexpected 'err' in result: {}", result);
        println!("{}", result)
    }

    // #[test]
    fn test_ai_extract() {
        let result = ai_extract("Apple Vision Pro is announced on 2021-10-18.");
        assert!(result.len() > 0);
        assert!(!result.contains("err"), "Unexpected 'err' in result: {}", result);
        println!("{}", result)
    }
}
