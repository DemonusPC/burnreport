import React from "react";
import { extractTabularNutrients } from "../../util/data/calculations";
import NutrientTableRow from "../../components/NutrientTableRow";
import { Nutrients } from "../../nutrients/nutrients";
import { Box, Text } from "grommet";
import EnergyRow from "./energy";

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

  return <><EnergyRow energy={nutrients.energy} amount={amount} /> {rowElements}</>;
};


export default NutrientTable;
