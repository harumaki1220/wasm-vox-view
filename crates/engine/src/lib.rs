use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
struct Comment {
    id: i32,
    author: String,
    text: String,
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
        let new_comment = Comment { id, author, text };
        self.items.push(new_comment);
    }

    pub fn pop_next_text(&mut self) -> Option<String> {
        if self.items.is_empty() {
            None
        } else {
            let comment = self.items.remove(0);
            Some(comment.text)
        }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! RustのWasmエンジンからの返答です。", name)
}
