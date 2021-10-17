import React from "react";
import { Text, Box } from "grommet";
import { BorderType } from "grommet/utils";

interface NutrientTableRowProps {
  row: NutrientRow;
}

export type NutrientRow = {
  level: number;
  name: string;
  highlight: boolean;
  amount?: number;
};

const levelToMargin = (level: number) => {
  switch (level) {
    case 1:
      return "small";
    case 2:
      return "medium";
    case 3:
      return "large";
    default:
      return "0";
  }
};

const highlightToWeight = (highlight: boolean): "bold" | "normal" => {
  if (highlight) {
    return "bold";
  }
  return "normal";
};

const highlightToBorder = (highlight: boolean): BorderType | undefined => {
  if (highlight) {
    return {
      color: "border",
      size: "1px",
      style: "solid",
      side: "top",
    };
  }
  return undefined;
};
const NutrientTableRow = ({ row }: NutrientTableRowProps) => {
  const amount: JSX.Element = (
    <Box fill alignSelf="end" align="end">
      <Text weight={highlightToWeight(row.highlight)}>
        {row.amount || 0} g{" "}
      </Text>
    </Box>
  );
  return (
    <Box
      direction="row"
      margin={{
        top: row.highlight ? "medium" : "small",
      }}
      border={highlightToBorder(row.highlight)}
      pad={row.highlight ? { top: "small" } : undefined}
    >
      <Box fill margin={{ left: levelToMargin(row.level) }}>
        <Text weight={highlightToWeight(row.highlight)}>{row.name} </Text>
      </Box>
      {amount || 0}
    </Box>
  );
};

export default NutrientTableRow;
