import {invoke} from "@tauri-apps/api/tauri";
import Spreadsheet from "x-data-spreadsheet";
import saveIcon from './assets/content-save.svg'
import {save} from "@tauri-apps/api/dialog";


window.addEventListener("DOMContentLoaded", async () => {
  const data: JSON = await invoke("serialize");

  new Spreadsheet("#sheet", {
    extendToolbar: {
      left: [
        {
          tip: 'Save',
          icon: saveIcon,
          onClick: async () => {
            const filePath = await save({
              filters: [{
                name: 'Save',
                extensions: ['xlsx']
              }]
            });
            if (filePath !== null) {
              await invoke("save", {filePath});
            }
          }
        }
      ],
    }
  })
    .loadData(data)
    .change(async data => {
      await invoke("save_cell", {payload: JSON.stringify(data)});
    });
});
