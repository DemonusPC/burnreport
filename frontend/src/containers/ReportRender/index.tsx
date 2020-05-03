import React from "react";
import { Product } from "../../util/schema/product";
import {
  Box,
  Heading,
  DataTable,
  Button,
} from "grommet";
import { saveAs } from 'file-saver';

import styled from "styled-components";
import NutrientTable from "../NutrientTable";
import NutrientBar from "../NutrientBar";
import { totalMacroInGrams } from "../../pages/Products";
import { displayRound, flattenProductList } from "../../util/data/calculations";
import { FlatProduct } from "../../util/schema/report";

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

const cols = [
  {
    property: "name",
    header: "Name",
    primary: true,
  },
  {
    property: "manufacturer",
    header: "Manufacturer",
  },
  {
    property: "kcal",
    header: "Energy (kcal)",
    render: (datum: FlatProduct) =>  displayRound(datum.kcal),
    // align: 'end',
  },
  {
    property: "kj",
    header: "Energy (kJ)",
    render: (datum: FlatProduct) =>  displayRound(datum.kj)
    // align: 'end',
  },
  {
    property: "carbohydrates",
    header: "Catbohydrates",
    render: (datum: FlatProduct) =>  displayRound(datum.carbohydrates)
    // align: 'end',
  },
  {
    property: "sugaer",
    header: "Sugar",
    render: (datum: FlatProduct) =>  displayRound(datum.sugar)
    // align: 'end',
  },
  {
    property: "addedSugaer",
    header: "Added Sugar",
    render: (datum: FlatProduct) =>  displayRound(datum.addedSugaer)
    // align: 'end',
  },
  {
    property: "fiber",
    header: "Fiber",
    render: (datum: FlatProduct) =>  displayRound(datum.fiber)
    // align: 'end',
  },
  {
    property: "starch",
    header: "Starch",
    render: (datum: FlatProduct) =>  displayRound(datum.starch)
    // align: 'end',
  },
  {
    property: "fat",
    header: "Fat",
    render:  (datum: FlatProduct) =>  displayRound(datum.fat)
    // align: 'end',
  },
  {
    property: "saturated",
    header: "Saturated",
    render: (datum: FlatProduct) =>  displayRound(datum.saturated)
    // align: 'end',
  },
  {
    property: "monounsaturated",
    header: "Monounsaturated",
    render: (datum: FlatProduct) =>  displayRound(datum.monounsaturated)
    // align: 'end',
  },
  {
    property: "trans",
    header: "Trans",
    render: (datum: FlatProduct) =>  displayRound(datum.trans)
    // align: 'end',
  },
  {
    property: "protein",
    header: "Protein",
    render: (datum: FlatProduct) =>  displayRound(datum.protein)
    // align: 'end',
  },
  {
    property: "salt",
    header: "Salt",
    render: (datum: FlatProduct) =>  displayRound(datum.salt)
    // align: 'end',
  },
];

interface StateProps {
  property: string;
  direction: "asc" | "desc";
}

const starterProps = () : StateProps => {
  return {
    property: "name",
    direction: "asc"
  }
}

const ReportRender = ({ result }: ReportResult) => {
  const [sort, setSort] = React.useState(starterProps());
  return (
    <>
    <Box>
      <Heading>Report</Heading>
      <Box direction="row-responsive">
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
          <NutrientTable product={result.total} amount={100} />
        </Box>

        <Box>
          <ProductName level={2}>Products consumed</ProductName>
          <DataTable
            columns={cols}
            data={flattenProductList(result.consumed)}
            sort={sort}
            onSort={setSort}
            resizeable
          />
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
