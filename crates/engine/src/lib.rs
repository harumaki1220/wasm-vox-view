use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct CommentQueue {
    items: Vec<Comment>,
}

// 一番外側の {} に対応する構造体
#[derive(Deserialize)]
struct YouTubeResponse {
    items: Vec<YouTubeItem>,
}

// items の中の {} に対応する構造体
#[derive(Deserialize)]
struct YouTubeItem {
    snippet: ChatSnippet,
}

// snippet の中の {} に対応する構造体
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatSnippet {
    display_message: String,
    author_channel_id: String,
}

#[derive(Clone, Debug, Serialize)]
struct Comment {
    id: i32,
    author: String,
    display_text: String,
    speech_text: String,
}

#[wasm_bindgen]
impl CommentQueue {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CommentQueue {
        CommentQueue { items: Vec::new() }
    }

    pub fn add_comment(&mut self, id: i32, author: String, text: String) {
        if text.contains("バカ") {
            return;
        }
        let processed_text = text.replace("w", "わら");
        let new_comment = Comment {
            id,
            author,
            display_text: text,
            speech_text: processed_text,
        };
        self.items.push(new_comment);
    }

    pub fn pop_next_text(&mut self) -> Option<String> {
        if self.items.is_empty() {
            None
        } else {
            let comment = self.items.remove(0);
            Some(serde_json::to_string(&comment).unwrap())
        }
    }
}

#[wasm_bindgen]
pub fn test_parse_youtube_json() -> String {
    // YouTubeが送ってくる本物そっくりのダミーJSON文字列
    let dummy_json = r#"{
        "items": [
            {
                "snippet": {
                    "displayMessage": "Rustからのパーステスト成功w",
                    "authorChannelId": "UC_dummy12345"
                }
            }
        ]
    }"#;

    // JSON文字列を、作った構造体に解凍（パース）してみる
    // from_str は serde_json の機能で、文字列を構造体に変換します
    match serde_json::from_str::<YouTubeResponse>(dummy_json) {
        Ok(parsed) => {
            // パースに成功したら、一番奥の display_message を取り出して返す
            let msg = parsed.items[0].snippet.display_message.clone();
            format!("大成功！取り出した文字: {}", msg)
        }
        Err(e) => {
            // もし構造体の設計が間違っていたらエラーメッセージを返す
            format!("パース失敗... 理由: {}", e)
        }
    }
}
