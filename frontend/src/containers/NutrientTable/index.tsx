import React from "react";
import {
  Table,
  TableHeader,
  TableRow,
  TableCell,
  TableBody,
  Text,
} from "grommet";
import { Product, emptyProduct } from "../../util/schema/product";
import {
  extractTabularNutrients,
  RawNutrientRow,
} from "../../util/data/calculations";

import styled from 'styled-components';

import NutrientTableRow from "../../components/NutrientTableRow";

interface NutrientTableProps {
  product: Product;
  amount: number
}
const columns = [
  {
    property: "macronutrient",
    header: "Macronutrient",
  },
  {
    property: "nutrient",
    header: "Nutrient",
  },
  {
    property: "amount",
    header: "Amount",
  },
];


const mapExtractedRows = (rows: Array<RawNutrientRow>) => {
  return rows.map((row) => (
    <NutrientTableRow row={row} />
  ));
};

const TableWidth = styled(Table)`
  min-width: 100%;
`;

const NutrientTable = ({ product, amount }: NutrientTableProps) => {
  const rows = extractTabularNutrients(product, amount);
  return (
      <TableWidth alignSelf="stretch">
        <TableHeader>
          <TableRow>
            {columns.map((c) => (
              <TableCell key={c.property} scope="col">
                <Text>{c.header}</Text>
              </TableCell>
            ))}
          </TableRow>
        </TableHeader>
        <TableBody>
            {mapExtractedRows(rows)}
        </TableBody>
      </TableWidth>
  );
};

NutrientTable.defaultProps = {
  product: emptyProduct(),
  amount: 100
};

export default NutrientTable;
