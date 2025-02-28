import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ReactHtmlParser from "react-html-parser";

function App() {
  const [result, setResult] = useState("");
  const [resultType, setResultType] = useState("");
  const [command, setCommand] = useState("");

  async function execute() {
    const resp: any = await invoke("execute", { command });
    setResult(resp["result"]);
    setResultType(resp["result_type"]);
  }

  return (
    <main>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          execute();
        }}
      >
        <input
          onChange={(e) => setCommand(e.currentTarget.value)}
          placeholder="Enter a FriCAS command..."
        />
        <button type="submit">Run</button>
      </form>

      {ReactHtmlParser(result)}
      <div style={{ float: "right" }}>{ReactHtmlParser(resultType)}</div>
    </main>
  );
}

export default App;
