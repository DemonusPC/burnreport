import React from "react";
import { Product } from "../../util/schema/product";
import {
  Box,
  Heading,
  Accordion,
  Button,
} from "grommet";
import { saveAs } from 'file-saver';

import styled from "styled-components";
import NutrientTable from "../NutrientTable";
import NutrientBar from "../NutrientBar";
import { totalMacroInGrams } from "../../pages/Products";
import { displayRound } from "../../util/data/calculations";
import ConsumedItem from "../ConsumedItem";

export interface ReportResult {
  timeDone: number;
  result: {
    total: Product;
    consumed: Product[];
  };
}

const Energy = styled(Heading)`
  font-size: 2em;
`;

const ProductName = styled(Heading)`
  width: 500px;
`;

const mapConsumed = (consumed: Product[]) => {
  return consumed.map((product: Product) => (
    <ConsumedItem {...product} />
  ))
}

const ReportRender = ({ result }: ReportResult) => {
  return (
    <>
    <Box>
      <Heading>Report</Heading>
      <Box direction="column">
        <Box margin={{ right: "xlarge" }}>
          <ProductName level={2}>Total consumed</ProductName>
          <NutrientBar
            total={totalMacroInGrams(result.total)}
            carbohydrates={result.total.carbohydrates.total}
            fat={result.total.fat.total}
            protein={result.total.protein.total}
          />
          <Energy level={4}>
            {" "}
            {displayRound(result.total.energy.kcal)} kcal /{" "}
            {displayRound(result.total.energy.kj)} kJ
          </Energy>
          <NutrientTable product={result.total} amount={100} baseUnit={1} />
        </Box>

        <Box>
          <ProductName level={2}>Products consumed</ProductName>
          <Accordion multiple>
            {mapConsumed(result.consumed)}
          </Accordion>
        </Box>
      </Box>
    </Box>
    <Box margin={{
      top: "xlarge"
    }} direction="row" gap="medium">
        <Button type="submit" label="Download as JSON" onClick={() => {
            const blob = new Blob([JSON.stringify(result, null, 2)], {type : 'application/json'});

            saveAs(blob, "report.json");
        }} />
        <Button type="reset" label="Reset" onClick={() => {
          window.location.reload();
        }} />
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
