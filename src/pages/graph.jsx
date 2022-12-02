import { Chart as ChartJS, CategoryScale, LinearScale, PointElement, LineElement, Title, Tooltip, Legend, } from 'chart.js';
import { Line } from 'react-chartjs-2';
import { invoke } from "@tauri-apps/api/tauri";
import { useState, useEffect } from "react";

let number = 0;

function graph() {
    const [idData, setIdData] = useState([]);
    const [tsidData, setTsidData] = useState([]);
    const [isLoading, setIsLoading] = useState(true);
    const [selectionPopulated, setSelectionPopulatedsetIsLoading] = useState(false);


    useEffect(() => {
        async function populateSel() {
            let command_string = "";
            await invoke("read_session_details").then((res) => command_string = res);

            command_string = command_string.replaceAll("[", '');
            command_string = command_string.replaceAll("]", '');
            command_string = command_string.replaceAll(" ", '')
            command_string = command_string.split(",");
            let filedata = [];
            for (let i = 0; i < command_string.length; i++) {
                filedata.push({ key: number++, id: command_string[i] });
            }
            setIdData(filedata);
            setIsLoading(false);
            setSelectionPopulatedsetIsLoading(true);
        };

        if (!selectionPopulated) {
            populateSel();
        }

    });


    async function populateTSIDSel(e) {
        let command_string = "";
        await invoke("get_tsid_from_id_struct", {idstring: e}).then((res) => command_string = res);

        command_string = command_string.replaceAll("[", '');
        command_string = command_string.replaceAll("]", '');
        command_string = command_string.replaceAll(" ", '')
        command_string = command_string.split(",");
        let filedata = [];
        for (let i = 0; i < command_string.length; i++) {
            filedata.push({ key: number++, id: command_string[i] });
        }
        setTsidData(filedata);
    };

    function load() {
        return (<option key="loading">Loading</option>)
    }

    function notload() {
        
    }

    async function selectOnChange(e) {
        console.log("Target Value: " + e.target.value);
        await populateTSIDSel(e.target.value);
    }

    return (
        <div className="content">
            <div className="invoke_div">
                <select onChange={(e) => selectOnChange(e)}>
                    {isLoading ? load() : notload()}
                    {idData.map(data => (<option key={data.key}>{data.id}</option>))}
                </select>
                <select>
                    {tsidData.map(data => (<option key={data.key}>{data.id}</option>))}
                </select>
            </div>

            <div className="graph_div">
                {/* <Line options={options} data={chartData} /> */}
            </div>
        </div>
    );
}

export default graph;