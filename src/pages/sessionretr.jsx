import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function session() {
    const [data, setData] = useState();
    const [id, setID] = useState();
    const [tsid, setTSID] = useState();

    async function retrieveData() {
        if(isNaN(parseInt(id)) || isNaN(parseInt(tsid))) {
            alert("Please enter a valid number");
            return;
        }

        await invoke("session_retrieve", {id: parseInt(id), tsid: parseInt(tsid)}).then((res) => setData(res)).catch((err) => setData(err));
    }

    function updateID(event) {
        setID(event.target.value);
    }

    function updateTSID(event) {
        setTSID(event.target.value);
    }

    return (
        <div className="session_content">
            <div className="title">Input the session ID and TSID</div>
            <div className="row">
                <input className="input" placeholder="Session ID" onChange={updateID}></input>
                <input className="input" placeholder="Session TSID"  onChange={updateTSID}></input>
            </div>
            <div className="invoke_div">
                <button className="invoke" onClick={() => retrieveData()}>
                    Retrieve data
                </button>
            </div>
            <div className="data">{data}</div>
        </div>
    );
}

export default session;