import React from "react";
import { Heading, Box, FileInput, Form, Button } from "grommet";

import ReportRender from "../../containers/ReportRender";

import ProductSelect, { emptyState, ProductSelectState, selectStateToConsumed } from "../../containers/ProductSelect";
import { ConsumedProduct, ConsumedRaw, emptyReport, Report, ReportResult } from "../../report/report";
import { postReport } from "../../util/data/requests";

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




const sendReport = (consumed: ConsumedProduct[], setReport: any) => {
  const report: Report = {
    consumed,
  };

  postReport(report).then((json: any) => {
    setReport({ completed: true, report: json });
  });
};


const Index = () => {
  const [report, setReport] = React.useState({
    completed: false,
    report: emptyReport(),
  });

  const [productState, setProductState] = React.useState<ProductSelectState>(emptyState());

  return (
    <Box pad="large" gridArea="main">
      {!report.completed ? (
        <>
          <Heading size="small">Create Report</Heading>
          <Form onSubmit={() => {
            sendReport(selectStateToConsumed(productState), setReport);
          }}>
            <ProductSelect state={productState} setState={setProductState} />

            <Box direction="row" gap="medium">
              <Button
                type="submit"
                primary
                label="Submit"
              />
              <Button
                type="reset"
                label="Reset"
                onClick={() => {
                  setProductState(emptyState());
                }}
              />
            </Box>
          </Form>
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
      )
      }
    </Box >
  );
};

export default Index;
