import React from "react";
import { Product, emptyProduct } from "../../util/schema/product";
import {
  extractTabularNutrients,
  RawNutrientRow,
} from "../../util/data/calculations";
import NutrientTableRow from "../../components/NutrientTableRow";

interface NutrientTableProps {
  product: Product;
  amount: number;
  baseUnit: number;
}

const mapExtractedRows = (rows: Array<RawNutrientRow>) => {
  return rows.map((row) => (
    <NutrientTableRow key={`${row.macronutrient}-${row.nutrient}`} row={row} />
  ));
};

const NutrientTable = ({ product, amount, baseUnit }: NutrientTableProps) => {
  const rows = extractTabularNutrients(product, amount, baseUnit);
  return (
      <>
            {mapExtractedRows(rows)}
      </>
  );
};

NutrientTable.defaultProps = {
  product: emptyProduct(),
  amount: 100
};

export default NutrientTable;
