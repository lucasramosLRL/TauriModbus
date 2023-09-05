import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const func = async (setPortsList) => { 
  setPortsList(await invoke('list_serial_ports'));
}

function App() {
  const [portsList, setPortsList] = useState("");

  return (
    <div className="container">
      <button onClick={() => func(setPortsList)}>List serial ports</button>
      <select>
        {portsList.length > 0 && portsList.map( port => <option key={port}>{port}</option>)} 
      </select>  
    </div>
  );
}

export default App;
