import React from "react";
import { Text, Box } from "grommet";

export interface RowProps {
  entity: string;
  amount: number;
  unit: string;
}

const Row = ({ entity, amount, unit }: RowProps) => {
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
        top: "small",
      }}
      direction="row"
      key={`${entity}`}
    >
      <Box fill>
        <Text>{entity}</Text>
      </Box>
      <Box fill alignSelf="end" align="end">
        <Text weight="bold">
          {amount} {unit}{" "}
        </Text>
      </Box>
    </Box>
  );
};

export default Row;
