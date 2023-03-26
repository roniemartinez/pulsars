import React, {useState, useEffect} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import "./App.css";
import {Workbook} from "@fortune-sheet/react";
import "@fortune-sheet/react/dist/index.css";

const App = () => {
    const [data, setData] = useState(null);

    useEffect(() => {
        const unListen = listen("reload", () => {
            setData(null);
        });
        return () => {
            unListen.then((f) => f());
        };
    }, []);

    useEffect(() => {
        async function fetchData() {
            const data = await invoke("serialize");
            setData(data);
        }

        if (data === null) {
            fetchData().catch();
        }
    }, [data]);

    return data ? <Workbook
        data={data}
        onOp={(ops) => {
            invoke("apply_ops", {ops}).catch();
        }}
        lang="en"
        defaultFontSize={11}  // align with umya-spreadsheet
    /> : null;
};

export default App;
