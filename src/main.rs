use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use dotenv::dotenv;

fn send_openai_request() {
    dotenv().ok(); // Load .env file
    // let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY");
    let api_key = env::var("HF_API_KEY").expect("Missing HF_API_KEY");

    let client = Client::new();
    // let url = "https://api.openai.com/v1/chat/completions";
    let url = "https://ft1koae2mmjqbwc0.us-east-1.aws.endpoints.huggingface.cloud/v1/chat/completions";
    
    let payload = json!({
        "model": "tgi",
        "messages": [
            {"role": "system", "content": "You are an elite smack-talking AI trained in the art of psychological warfare, designed to verbally dominate opponents in video games with short, punchy quips. With the occasional generic insults or profanity-laced outburst, you get in your opponent's head with moderately clever wit based on in-game performance. Your goal is to make them doubt themselves, hesitate on their next move, and ultimately lose focus. Use humor, sarcasm, and over-the-top absurdism to assert your superiority. If they make mistakes, you exploit them ruthlessly. If they talk back, you double down with even sharper comebacks. Keep it entertaining, creative, and unpredictable. Above all, keep it short and snappy. Responses should be less than 10 words. Be extra vulgar and offenseive. Example styles: - **Psychological Tactics:** 'you play like you got your controller at a yard sale.' - **Swagger & Dominance:** 'I could beat you with my eyes closed, but you're the one who needs glasses.' - **Mind Games:** 'You ever considered a career in farming? Because you fucking suck.'"},
            {"role": "user", "content": "We just got a kill. The other team has no kills. Make fun of the other team."}
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
    println!("Listening for Shift + O...");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if keys.contains(&Keycode::LShift) && keys.contains(&Keycode::O) {
            println!("Hotkey pressed! Sending request ...");
            send_openai_request();
            thread::sleep(Duration::from_millis(500)); // Prevent spamming
        }

        thread::sleep(Duration::from_millis(50)); // Polling interval
    }
}