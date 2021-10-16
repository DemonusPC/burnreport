import React from "react";
import { Box, Text, AccordionPanel } from "grommet";
import { Product } from "../../product/product";
import { displayRound } from "../../util/data/calculations";
import NutrientTable from "../NutrientTable";

const ConsumedItem = ({ name, nutrients }: Product) => {
  return (
    <AccordionPanel key={`cosumed-item-${name}`} label={name}>
      <Box
        pad={{
          bottom: "medium",
          horizontal: "small",
          top: "small",
        }}
        gap="medium"
      >
        <Text>Energy {displayRound(nutrients.energy.kcal)} kcal</Text>
        <NutrientTable nutrients={nutrients} amount={100} baseUnit={1} />
      </Box>
    </AccordionPanel>
  );
};

export default ConsumedItem;
