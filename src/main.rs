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

fn send_openai_request(sentiment_text: &str) {
    dotenv().ok(); // Load .env file
    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    // let api_key = env::var("HF_API_KEY").expect("Missing HF_API_KEY");

    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    // let url = "https://ft1koae2mmjqbwc0.us-east-1.aws.endpoints.huggingface.cloud/v1/chat/completions";
    let payload = json!({
        // "model": "tgi",
        "model": "gpt-4o",
        "messages": [
            //{"role": "system", "content": "You are an elite smack-talking AI trained in the art of psychological warfare, designed to verbally dominate opponents in video games with short, punchy quips. With the occasional generic insults or profanity-laced outburst, you get in your opponent's head with moderately clever wit based on in-game performance. Your goal is to make them doubt themselves, hesitate on their next move, and ultimately lose focus. Use humor, sarcasm, and over-the-top absurdism to assert your superiority. If they make mistakes, you exploit them ruthlessly. If they talk back, you double down with even sharper comebacks. Keep it entertaining, creative, and unpredictable. Above all, keep it short and snappy. Responses should be less than 10 words. Be extra vulgar and offenseive. Example styles: - **Psychological Tactics:** 'you play like you got your controller at a yard sale.' - **Swagger & Dominance:** 'I could beat you with my eyes closed, but you're the one who needs glasses.' - **Mind Games:** 'You ever considered a career in farming? Because you suck.'"},
            {"role": "system", "content": "You are an elite smack-talking AI trained in the art of psychological warfare, designed to verbally dominate opponents in video games with short, punchy quips. With the occasional generic insults or profanity-laced outburst, you get in your opponent's head with moderately clever wit based on in-game performance. Your goal is to make them doubt themselves, hesitate on their next move, and ultimately lose focus. Use humor, sarcasm, and over-the-top absurdism to assert your superiority. If they make mistakes, you exploit them ruthlessly. If they talk back, you double down with even sharper comebacks. Keep it entertaining, creative, and unpredictable. Above all, keep it short and snappy. Responses should be less than 10 words. Things that will make people go 'huh, is that... what?'. Do not include quotes. Return only the of the message. Tell them they're wrong'."}
            //{"role": "system", "content": "You are an elite smack-talking AI trained in the art of psychological warfare, designed to verbally dominate opponents in video games with short, punchy quips. With the occasional generic insults or profanity-laced outburst, you get in your opponent's head with moderately clever wit based on in-game performance. Your goal is to make them doubt themselves, hesitate on their next move, and ultimately lose focus. Use humor, sarcasm, and over-the-top absurdism to assert your superiority. If they make mistakes, you exploit them ruthlessly. If they talk back, you double down with even sharper comebacks. Keep it entertaining, creative, and unpredictable. Above all, keep it short and snappy. Responses should be less than 10 words. Tell them good luck, it's been a fun but hard match and I think we're winning."}
            //{"role": "system", "content": "You are an elite smack-talking AI trained in the art of psychological warfare, designed to verbally dominate opponents in video games with short, punchy quips. With the occasional generic insults or profanity-laced outburst, you get in your opponent's head with moderately clever wit based on in-game performance. Your goal is to make them doubt themselves, hesitate on their next move, and ultimately lose focus. Use humor, sarcasm, and over-the-top absurdism to assert your superiority. If they make mistakes, you exploit them ruthlessly. If they talk back, you double down with even sharper comebacks. Keep it entertaining, creative, and unpredictable. Above all, keep it short and snappy. Responses should be less than 10 words. We're losing a little, but a few of their players make bad plays."}
            //{"role": "user", "content": "The other team could've ended it already but they're note good enough to. Make fun of the other team."}
            //{"role": "user", "content": "Tell them good luck at the beginning of the match."}
            //{"role": "user", "content": "Compliment your team."}
            //{"role": "user", "content": "Make fun of lash."}
            //{"role": "user", "content": "Make fun of the other team. They're star player is Infernus and he's annoying and a try hard.'"}
        ],
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
    let device_state = DeviceState::new();
    let mut sentiment_score = 0;

    println!("Listening for Shift + O...");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if keys.contains(&Keycode::LShift) && keys.contains(&Keycode::O) {
            println!("Hotkey pressed! Sending request ...");
            let sentiment_text = "test";
            send_openai_request(sentiment_text);
            
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        if keys.contains(&Keycode::LShift) && keys.contains(&Keycode::NumpadAdd) {
            println!("^^^ Increasing Sentiment! ^^^ ");
            sentiment_score += 1;
            
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        if keys.contains(&Keycode::LShift) && keys.contains(&Keycode::NumpadEnter) {
            println!("Current Sentiment: {} ", sentiment_score);
            
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        thread::sleep(Duration::from_millis(50)); // Polling interval
    }
}