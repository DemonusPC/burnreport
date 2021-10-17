import React from "react";
import { Product } from "../../product/product";
import { Box, Heading, Accordion, Button } from "grommet";
import { saveAs } from "file-saver";

import styled from "styled-components";
import NutrientTable from "../NutrientTable";
import { displayRound } from "../../util/data/calculations";
import ConsumedItem from "../ConsumedItem";
import AdditionalTable from "../AdditionalTable";
import { vitaminsToRow } from "../../nutrients/vitamins";
import {
  nutrientsToBarTotal,
  nutrientsToBarValues,
  polyunsaturatedToBarTotal,
  polyunsaturatedToBarValues,
} from "../../nutrients/nutrients";
import Bar from "../Bar";
import { ReportResult } from "../../report/report";

const Energy = styled(Heading)`
  font-size: 2em;
`;

const mapConsumed = (consumed: Product[]) => {
  return consumed.map((product: Product) => <ConsumedItem {...product} />);
};

const ReportRender = ({ result }: ReportResult) => {
  return (
    <>
      <Box>
        <Heading>Report</Heading>
        <Box direction="column">
          <Heading level={2}>Total consumed</Heading>
          <Bar
            data={result.total}
            mapToBarValues={nutrientsToBarValues}
            calculateTotal={nutrientsToBarTotal}
          />
          <Energy level={4}>
            {displayRound(result.total.energy.kcal)} kcal
          </Energy>
          <NutrientTable nutrients={result.total} amount={100} baseUnit={1} />
          <Heading level={2}>Vitamins</Heading>
          <AdditionalTable
            entity={result.total.vitamins}
            mapper={vitaminsToRow}
            unit={"mg"}
          />

          <Heading level={3}> Omega 3 to Omega 6</Heading>
          {result.total.fat.unsaturated &&
            result.total.fat.unsaturated.poly && (
              <Bar
                data={result.total.fat.unsaturated.poly}
                mapToBarValues={polyunsaturatedToBarValues}
                calculateTotal={polyunsaturatedToBarTotal}
              />
            )}
          <Heading level={2}>Products consumed</Heading>
          <Accordion multiple>{mapConsumed(result.consumed)}</Accordion>
        </Box>
      </Box>
      <Box
        margin={{
          top: "xlarge",
        }}
        direction="row"
        gap="medium"
      >
        <Button
          type="submit"
          label="Download as JSON"
          onClick={() => {
            const blob = new Blob([JSON.stringify(result, null, 2)], {
              type: "application/json",
            });

            saveAs(blob, "report.json");
          }}
        />
        <Button
          type="reset"
          label="Reset"
          onClick={() => {
            window.location.reload();
          }}
        />
      </Box>
    </>
  );
};

ReportRender.defaultProps = {
  timeDone: Date.now(),
  result: {
    total: {
      kcal: 0,
    },
    consumed: [],
  },
};

export default ReportRender;
