import React from "react";
import { Heading, Box, FileInput } from "grommet";
import { emptyProduct } from "../../util/schema/product";

import ReportRender, { ReportResult } from "../../containers/ReportRender";

import ReportForm from "../../containers/ReportForm";

const emptyReport = (): ReportResult => {
  return {
    timeDone: Date.now(),
    result: {
      total: emptyProduct(),
      consumed: [],
    },
  };
};

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
    <>
      {!report.completed ? (
        <Box pad="large" align="center">
          <ReportForm setReportFunction={setReport} />
          <Box>
            <Heading>Import from file</Heading>
            <FileInput
              onChange={(e) => {
                if (e.target.files) {
                  fileChosen(e.target.files[0], setReport);
                }
              }}
            />
          </Box>
        </Box>
      ) : (
        <Box pad="large">
          <ReportRender result={report.report.result} />
        </Box>
      )}
    </>
  );
};

export default Index;
