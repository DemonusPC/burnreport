import React from "react";
import { Heading, Box, FileInput } from "grommet";

import ReportRender, { ReportResult } from "../../containers/ReportRender";

import ReportForm from "../../containers/ReportForm";
import { emptyNutrients } from "../../util/schema/product";

const emptyReport = (): ReportResult => {
  return {
    timeDone: Date.now(),
    result: {
      total: emptyNutrients(),
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
    <Box pad="large" gridArea="main">
      {!report.completed ? (
        <>
          <ReportForm setReportFunction={setReport} />
          <Box>
            <Heading size="small">Import from file</Heading>
            <FileInput
              onChange={(e) => {
                if (e.target.files) {
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
