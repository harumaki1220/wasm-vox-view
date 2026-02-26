import { useEffect, useState } from "react";
import init, { CommentQueue } from "engine";
import wasmUrl from "engine/engine_bg.wasm?url";

let globalQueue: CommentQueue | null = null;
let initPromise: Promise<any> | null = null;

function App() {
  const [isReady, setIsReady] = useState(false);
  const [currentMessage, setCurrentMessage] =
    useState<string>("まだコメントはありません");

  useEffect(() => {
    const loadWasm = async () => {
      try {
        if (!initPromise) {
          initPromise = init(wasmUrl);
        }
        await initPromise;

        if (!globalQueue) {
          globalQueue = new CommentQueue();
        }
        setIsReady(true);
      } catch (error) {
        console.error("Wasm初期化エラー:", error);
      }
    };
    loadWasm();
  }, []);

  // コメントを追加する関数
  const handleAddComment = () => {
    if (!globalQueue) return;

    globalQueue.add_comment(1, "テストユーザー", "こんにちは！Wasm！");
    console.log("キューにコメントを追加しました");
  };

  // キューからコメントを取り出す関数
  const handlePopComment = () => {
    if (!globalQueue) return;

    const result = globalQueue.pop_next_text();

    if (result !== undefined) {
      setCurrentMessage(result);
    } else {
      setCurrentMessage("キューは空っぽです");
    }
  };

  if (!isReady) {
    return (
      <div className="min-h-screen bg-gray-900 text-white flex items-center justify-center">
        Wasmを読み込み中...
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-4">
      <div className="bg-gray-800 border border-gray-700 rounded-xl p-8 shadow-2xl max-w-md w-full text-center space-y-6">
        <h1 className="text-2xl font-bold">コメビュツール テスト</h1>

        {/* 現在読み上げている（取り出した）コメントの表示エリア */}
        <div className="bg-gray-900 py-4 px-4 rounded-lg border border-gray-700 h-24 flex items-center justify-center">
          <p className="text-lg font-medium text-cyan-400">{currentMessage}</p>
        </div>

        {/* 操作ボタン */}
        <div className="flex gap-4 justify-center">
          <button
            onClick={handleAddComment}
            className="bg-blue-600 hover:bg-blue-500 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            追加する
          </button>
          <button
            onClick={handlePopComment}
            className="bg-pink-600 hover:bg-pink-500 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            取り出す
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
