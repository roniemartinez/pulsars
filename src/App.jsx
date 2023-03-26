import React, {useState, useEffect} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import "./App.css";
import {Workbook} from "@fortune-sheet/react";
import "@fortune-sheet/react/dist/index.css";

const toolbarItems = [
    "undo",
    "redo",
    // "format-painter",
    // "clear-format",
    "|",
    // "currency-format",
    // "percentage-format",
    // "number-decrease",
    // "number-increase",
    "format",
    "font",
    "font-size",
    "|",
    "bold",
    "italic",
    "strike-through",
    // "underline",
    "|",
    "font-color",
    // "background",
    // "border",
    // "merge-cell",
    // "|",
    // "horizontal-align",
    // "vertical-align",
    // "text-wrap",
    // "text-rotation",
    // "|",
    // "freeze",
    // "sort",
    // "image",
    // "comment",
    // "quick-formula",
];

const cellContextMenu = [
    "copy",
    "paste",
    "|",
    "insert-row",
    "insert-column",
    "delete-row",
    "delete-column",
    "delete-cell",
    "hide-row",
    "hide-column",
    // "clear",
    // "sort",
    // "filter",
    // "chart",
    // "image",
    // "link",
    // "data",
    // "cell-format"
];

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
        toolbarItems={toolbarItems}
        cellContextMenu={cellContextMenu}
    /> : null;
};

export default App;
