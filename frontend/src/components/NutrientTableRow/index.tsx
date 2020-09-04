import React from "react";
import { RawNutrientRow } from "../../util/data/calculations";
import { Text, Box } from "grommet";

interface NutrientTableRowProps {
  row: RawNutrientRow;
}

const NutrientTableRow = ({ row }: NutrientTableRowProps) => {
  // Nutrient table row groups by Macro Nutrient
  // Setting it as a main category
  if (row.nutrient === "total") {
    return (
      <Box
        border={{
          color: "border",
          size: "1px",
          style: "solid",
          side: "top",
        }}
        margin={{
          top: "small",
        }}
        pad={{
          top: "small"
        }}
        direction="row"
        key={`${row.macronutrient}-${row.nutrient}`}
      >
        <Box fill>
          <Text weight="bold">{row.macronutrient}</Text>
        </Box>
        <Box fill alignSelf="end" align="end">
          <Text weight="bold">{row.amount} g </Text>
        </Box>
      </Box>
    );
  }
  return (
    <Box
      direction="row"
      margin={{
        top: "xsmall",
      }}
    >
      <Box fill margin={{ left: "large" }}>
        {row.nutrient}
      </Box>
      <Box fill alignSelf="end" align="end">
        {row.amount} g
      </Box>
    </Box>
  );
};

export default NutrientTableRow;
