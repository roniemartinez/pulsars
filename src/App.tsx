import { Workbook } from "@fortune-sheet/react";
import "@fortune-sheet/react/dist/index.css";
import { useMemo } from "react";
import "./App.css";
import { cellContextMenu, defaultFontSize, toolbarItems } from "./config";
import { useWorkbook } from "./hooks/useWorkbook";
import { createCustomToolbarItems } from "./toolbar";

export default function App() {
  const { sheets, error, applyOps, reportError } = useWorkbook();
  const customToolbarItems = useMemo(() => createCustomToolbarItems(reportError), [reportError]);

  if (error !== null) {
    return <div className="error">{error}</div>;
  }

  if (sheets === null) {
    return null;
  }

  return (
    <Workbook
      data={sheets}
      onOp={applyOps}
      lang="en"
      defaultFontSize={defaultFontSize}
      toolbarItems={toolbarItems}
      cellContextMenu={cellContextMenu}
      customToolbarItems={customToolbarItems}
    />
  );
}
