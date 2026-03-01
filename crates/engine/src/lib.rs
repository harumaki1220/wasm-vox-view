use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize)]
struct Comment {
    id: i32,
    author: String,
    display_text: String,
    speech_text: String,
}

#[wasm_bindgen]
struct CommentQueue {
    items: Vec<Comment>,
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
pub fn greet(name: &str) -> String {
    format!("Hello, {}! RustのWasmエンジンからの返答です。", name)
}
