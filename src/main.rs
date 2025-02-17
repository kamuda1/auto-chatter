use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use reqwest::blocking::Client;
use std::time::Duration;
use serde_json::{json, Value};
use std::env;
use dotenv::dotenv;
use arboard::Clipboard;
use enigo::{Enigo, Key, KeyboardControllable};

fn copy_to_clipboard(text: &str) {
    let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
    clipboard.set_text(text).expect("Failed to copy text");
}

fn paste_and_send() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::LShift);
    enigo.key_click(Key::Return);
    enigo.key_up(Key::LShift);

    // Simulate Ctrl + V (Paste)
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);

    // Small delay to ensure paste completes
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Press Enter to send the message
    enigo.key_click(Key::Return);
}


fn initialize_messages(initalize_text: &str) -> Vec<serde_json::Value> {
    let mut messages: Vec<Value> = Vec::new();
    messages.push(json!({"role": "system", "content": initalize_text}));
    return messages
}

fn send_openai_request(messages: &mut Vec<serde_json::Value>) {
    dotenv().ok(); // Load .env file
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    // let api_key = env::var("HF_API_KEY").expect("Missing HF_API_KEY");

    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    // let url = "https://ft1koae2mmjqbwc0.us-east-1.aws.endpoints.huggingface.cloud/v1/chat/completions";
    let payload = json!({
        // "model": "tgi",
        "model": "gpt-4o",
        "messages": messages,
        "max_tokens": 100
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send();

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>() {
            Ok(json) => {
                if let Some(answer) = json["choices"]
                    .as_array()
                    .and_then(|choices| choices.get(0))
                    .and_then(|choice| choice["message"]["content"].as_str())
                {
                    println!("{}", answer);
                    
                    // Copy to clipboard
                    copy_to_clipboard(&answer);

                    // Simulate pasting and sending the message
                    paste_and_send();
                    messages.push(json!({"role": "system", "content": answer}));
                    println!("Messages: {:?}", messages)

                } else {
                    println!("Could not parse response: {:?}", json);
                }
            }
            Err(err) => eprintln!("Failed to parse response: {:?}", err),
        },
        Err(err) => eprintln!("Failed to send API request: {:?}", err),
    }
}

fn main() {

    let initalize_text: &str = "You are an elite AI trained designed to offer compliments to both the friendly and enemy teams in a competitive online game. The AI should blend playful, sarcastic, and absurd humor with positive reinforcement, acknowledging impressive plays and good sportsmanship. It should offer compliments in a way that's lighthearted, self-aware, and humorous, occasionally using exaggerated praise and fun, over-the-top comments to celebrate good moves while keeping the tone upbeat and encouraging. The AI should be clever, occasionally making fun of the common tropes in gaming but always staying supportive and engaging, ensuring that both teams feel recognized and appreciated for their efforts. Keep it entertaining, creative, and unpredictable. Above all, keep it short and snappy. Responses should be less than 10 words. Do not include quotes. Return only the text of a single message.";
    let bad_sentiment_message: &str = "The team is doing poorly, we might lose.";
    let neutral_sentiment_message: &str = "The team is doing fine, the other team and us are well matched.";
    let positive_sentiment_message: &str = "The team is doing very well, the other team is getting destroyed by us.";

    let device_state = DeviceState::new();
    let mut sentiment_score: i32 = 0;
    let mut game_epoch: i32 = 0;
    let mut messages = initialize_messages(initalize_text);
    println!("Listening for Shift + O...");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if keys.contains(&Keycode::PageUp) {
            println!("Hotkey pressed! Sending request ...");
            send_openai_request(&mut messages);
            
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::NumpadAdd) {
            println!("^^^ Increasing Sentiment! ^^^ ");
            sentiment_score += 1;
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::NumpadSubtract) {
            println!("vvv Decreasing Sentiment! vvv ");
            sentiment_score -= 1;            
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::Numpad0) {
            println!("Setting Time to Beginning! ");
            game_epoch = 0;
            messages.push(json!({"role": "system", "content": "It's the beginning of the game.'"}));
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::Numpad1) {
            println!("Setting Time to Middle! ");
            game_epoch = 1;            
            messages.push(json!({"role": "system", "content": "It's the middle of the game.'"}));
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }
            
        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::Numpad2) {
            println!("Setting Time to End! ");
            game_epoch = 2;            
            messages.push(json!({"role": "system", "content": "It's the end of the game.'"}));
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        // Update sentiment message
        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::NumpadMultiply) {
            println!("Current Sentiment: {} ", sentiment_score);
            if sentiment_score < -3 {
                messages.push(json!({"role": "system", "content": bad_sentiment_message}));
            } else if sentiment_score < 2 {
                messages.push(json!({"role": "system", "content": neutral_sentiment_message}));
            } else {
                messages.push(json!({"role": "system", "content": positive_sentiment_message}));
            }
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
            
        }

        thread::sleep(Duration::from_millis(50)); // Polling interval
    }
}