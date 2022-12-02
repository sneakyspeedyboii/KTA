import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function App() {
  const [data, setData] = useState("");
  const [testCMD, settestCMD] = useState("");
  
  async function updateData() {
    await invoke("live_data_fetch").then((res) => setData(res)).catch((err) => setData(err));
  }

  async function read_sessions() {
    await invoke("session_id_read").then((res) => settestCMD(res)); //maybe proof of concept
  }

  async function check_new_sessions() {
    await invoke("session_update").then((res) => settestCMD(res)); //need
  }

  async function download_all_sessions() {
    await invoke("session_detail_download").then((res) => settestCMD(res)); //need
  }

  return (
    <div className="index_content"> 
      <div className="row">
          <button className="invoke" onClick={ () => updateData() }>
            Grab live data
          </button>
          <button className="invoke" onClick={ () => check_new_sessions() }>
            Update Sessions
          </button>
        </div>
        <div className="row">
          <button className="invoke" onClick={ () => download_all_sessions() }>
            Download all session stats
          </button>
          <button className="invoke" onClick={ () => read_sessions() }>
            Read Sessions
          </button>
        </div>
        <p className="info">{data}</p>
        <p className="info">{testCMD}</p>
        
    </div>
  );
}

export default App;