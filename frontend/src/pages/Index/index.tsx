import React from "react";
import { Heading, Box, FileInput } from "grommet";

import ReportRender from "../../containers/ReportRender";

import ReportForm from "../../containers/ReportForm";
import { emptyReport, ReportResult } from "../../report/report";

const fileChosen = (file: any | undefined, setReport: any) => {
  const reader = new FileReader();
  reader.onloadend = (e: any) => {
    const content = reader.result;
    if (content) {
      const parsed: ReportResult = JSON.parse(content.toString());
      setReport({
        completed: true,
        report: { result: parsed },
      });
    }
  };

  reader.readAsText(file);
};

const Index = () => {
  const [report, setReport] = React.useState({
    completed: false,
    report: emptyReport(),
  });

  return (
    <Box pad="large" gridArea="main">
      {!report.completed ? (
        <>
          <ReportForm setReportFunction={setReport} />
          <Box>
            <Heading size="small">Import from file</Heading>
            <FileInput
              onChange={(e) => {
                if (e && e.target.files) {
                  fileChosen(e.target.files[0], setReport);
                }
              }}
            />
          </Box>
        </>
      ) : (
        <ReportRender result={report.report.result} />
      )}
    </Box>
  );
};

export default Index;
