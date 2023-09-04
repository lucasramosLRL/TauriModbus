import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const func = async (setJson) => { 
  setJson(await invoke('list_serial_ports'));
}

function App() {
  const [json, setJson] = useState();

  return (
    <div className="container">
      <div>{json}</div>  
      <button onClick={() => func(setJson)}>Click</button>
    </div>
  );
}

export default App;
