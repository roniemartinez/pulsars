import React, {useState, useEffect} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import "./App.css";
import {Workbook} from "@fortune-sheet/react";
import "@fortune-sheet/react/dist/index.css";
import {save, open} from "@tauri-apps/api/dialog";

const customToolbarItems = [
    {
        key: "open",
        tooltip: "Open",
        onClick: async () => {
            const filePath = await open({
                multiple: false,
                filters: [{
                    name: "Open",
                    extensions: ["xlsx"]
                }]
            });
            if (filePath !== null) {
                await invoke("open", {filePath});
            }
        },
        selected: false,
        icon: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
            <path
                d="M14,11C12.34,11 11,9.66 11,8V4H7C5.9,4 5,4.9 5,6V19C5,20.1 5.9,21 7,21H16C17.1,21 18,20.1 18,19V11H14M12,8C12,9.1 12.9,10 14,10H17.59L12,4.41V8M7,3H12L19,10V19C19,20.66 17.66,22 16,22H7C5.34,22 4,20.66 4,19V6C4,4.34 5.34,3 7,3Z"/>
        </svg>
    },
    {
        key: "save",
        tooltip: "Save",
        onClick: async () => {
            const filePath = await save({
                filters: [{
                    name: "Save",
                    extensions: ["xlsx"]
                }]
            });
            if (filePath !== null) {
                await invoke("save", {filePath});
            }
        },
        selected: false,
        icon: <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24">
            <path
                d="M6,4H16.59L20,7.41V18C20,19.66 18.66,21 17,21H6C4.34,21 3,19.66 3,18V7C3,5.34 4.34,4 6,4M6,5C4.9,5 4,5.9 4,7V18C4,19.1 4.9,20 6,20H17C18.1,20 19,19.1 19,18V7.91L16.09,5H15V9L15,10H6V9L6,5M7,5V9H14V5H7M12,12C13.66,12 15,13.34 15,15C15,16.66 13.66,18 12,18C10.34,18 9,16.66 9,15C9,13.34 10.34,12 12,12M12,13C10.9,13 10,13.9 10,15C10,16.1 10.9,17 12,17C13.1,17 14,16.1 14,15C14,13.9 13.1,13 12,13Z"/>
        </svg>
    }
];

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
        customToolbarItems={customToolbarItems}
    /> : null;
};

export default App;
