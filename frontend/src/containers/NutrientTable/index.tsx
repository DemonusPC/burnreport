import React from "react";
import { Nutrients, emptyProduct } from "../../product/product";
import { extractTabularNutrients } from "../../util/data/calculations";
import NutrientTableRow from "../../components/NutrientTableRow";

interface NutrientTableProps {
  nutrients: Nutrients;
  amount: number;
  baseUnit: number;
}

const NutrientTable = ({ nutrients, amount, baseUnit }: NutrientTableProps) => {
  const rows = extractTabularNutrients(nutrients, amount, baseUnit);

  const rowElements = rows.map((row) => (
    <NutrientTableRow key={`${row.name}-${row.level}`} row={row} />
  ));

  return <>{rowElements}</>;
};

NutrientTable.defaultProps = {
  product: emptyProduct(),
  amount: 100,
};

export default NutrientTable;
