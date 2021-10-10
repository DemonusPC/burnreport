import React from "react";
import { Box, Text, AccordionPanel } from "grommet";
import { Product } from "../../util/schema/product";
import { displayRound } from "../../util/data/calculations";
import NutrientTable from "../NutrientTable";

// const shortName = (name: string) : string => {
//   const max = 15
//   if(name.length > max) {
//     return name.slice(0, max - 3) + "...";
//   }
//   return name;
// }

const ConsumedItem = ({ id, name, unit, nutrients: nutrition }: Product) => {
  const product: Product = {
    id,
    name,
    nutrients: nutrition,
    unit,
  };
  return (
    <AccordionPanel label={name} key={name}>
      <Box
        pad={{
          bottom: "medium",
          horizontal: "small",
          top: "small",
        }}
        gap="medium"
      >
        <Text>Energy {displayRound(nutrition.energy.kcal)} kcal</Text>
        {/* <NutrientTable product={product} amount={100} baseUnit={1} /> */}
      </Box>
    </AccordionPanel>
  );
};

export default ConsumedItem;
