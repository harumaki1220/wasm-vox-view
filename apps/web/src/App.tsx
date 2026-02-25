import { useEffect, useState } from "react";
import init, { greet } from "engine";
import wasmUrl from "engine/engine_bg.wasm?url";

function App() {
  const [message, setMessage] = useState<string>("Wasmを読み込み中...");

  useEffect(() => {
    const loadWasm = async () => {
      try {
        await init(wasmUrl);
        const result = greet("React開発者");
        setMessage(result);
      } catch (error) {
        console.error("Wasmの初期化に失敗しました:", error);
        setMessage("エラーが発生しました。");
      }
    };
    loadWasm();
  }, []);

  return (
    <div className="min-h-screen bg-gray-900 text-white flex flex-col items-center justify-center p-4">
      <div className="bg-gray-800 border border-gray-700 rounded-xl p-8 shadow-2xl max-w-md w-full text-center">
        <h1 className="text-3xl font-extrabold mb-4 text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-blue-500">
          Wasm + React
        </h1>
        <p className="text-lg font-medium text-gray-300 bg-gray-900 py-3 px-4 rounded-lg border border-gray-700">
          {message}
        </p>
      </div>
    </div>
  );
}

export default App;
