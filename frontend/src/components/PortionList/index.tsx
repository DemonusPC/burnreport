import React from "react";
import { Text, Button, Box } from "grommet";
import { Portion } from "../../util/schema/product";
import { Close } from "grommet-icons";

interface PortionTableProps {
  productId: number;
  portions: Portion[];
  removeFunction: (portionName: string) => Promise<void>;
}

const PortionList = ({ portions, removeFunction }: PortionTableProps) => {
  if (portions.length === 0) {
    return <Text>No portion specified</Text>;
  }

  return (
    <>
      {portions.map((portion: Portion) => {
        return (
          <Box
            pad="medium"
            direction="row"
            justify="between"
            border="all"
            round={"4px"}
            key={portion.name}
            margin={{
              bottom: "small",
            }}
          >
            <Box width="20%">
              <Text weight="bold">{portion.name}</Text>
            </Box>

            <Text>{portion.grams} g</Text>
            <Button
              plain={true}
              hoverIndicator={true}
              size="medium"
              icon={<Close />}
              onClick={async () => {
                await removeFunction(portion.name);
              }}
            />
          </Box>
        );
      })}
    </>
  );
};

export default PortionList;
