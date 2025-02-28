import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { MathJax, MathJaxContext } from "better-react-mathjax";

const nilOutput = "$$nil$$";
const latexHelper = (input: string) => {
  const tmp = input
    .replace("$$", "$$$$\\let\\sp=^\\let\\sb=_\\let\\leqno=\\;\n")
    .split("$$");
  return [`$$${tmp[1]}$$`, tmp[2].replace(" ->", "")];
};

function App() {
  const [result, setResult] = useState("");
  const [resultType, setResultType] = useState("");
  const [command, setCommand] = useState("");

  async function execute() {
    const original: string = await invoke("execute", { command });
    if (!original.includes("$$")) {
      setResult(nilOutput);
      setResultType(nilOutput);
      return;
    }

    const [result, type] = latexHelper(original);
    setResult(result);
    setResultType(type);
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

      <MathJaxContext>
        <MathJax>{result}</MathJax>
        <div style={{ float: "right" }}>
          <MathJax>{resultType}</MathJax>
        </div>
      </MathJaxContext>
    </main>
  );
}

export default App;
