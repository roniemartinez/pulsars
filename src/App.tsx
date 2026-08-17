import { Workbook } from "@fortune-sheet/react";
import "@fortune-sheet/react/dist/index.css";
import { useMemo } from "react";
import "./App.css";
import { cellContextMenu, defaultFontSize, toolbarItems } from "./config";
import { useWorkbook } from "./hooks/useWorkbook";
import { createCustomToolbarItems } from "./toolbar";

export default function App() {
  const { sheets, revision, loadError, actionError, applyOps, reportError, dismissError } = useWorkbook();
  const customToolbarItems = useMemo(() => createCustomToolbarItems(reportError), [reportError]);

  if (loadError !== null) {
    return <div className="load-error">{loadError}</div>;
  }

  if (sheets === null) {
    return null;
  }

  return (
    <>
      {actionError !== null && (
        <div className="action-error" role="alert">
          <span>{actionError}</span>
          <button type="button" onClick={dismissError}>
            Dismiss
          </button>
        </div>
      )}
      <Workbook
        key={revision}
        data={sheets}
        onOp={applyOps}
        lang="en"
        defaultFontSize={defaultFontSize}
        toolbarItems={toolbarItems}
        cellContextMenu={cellContextMenu}
        customToolbarItems={customToolbarItems}
      />
    </>
  );
}
