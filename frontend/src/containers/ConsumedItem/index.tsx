import React from "react";
import styled from "styled-components";
import { Box, Text, AccordionPanel } from "grommet";
import { Product } from "../../util/schema/product";
import { displayRound } from "../../util/data/calculations";

// const shortName = (name: string) : string => {
//   const max = 15
//   if(name.length > max) {
//     return name.slice(0, max - 3) + "...";
//   }
//   return name;
// }

const ConsumedItem = ({
  name,
  energy,
  carbohydrates,
  fat,
  protein,
  salt,
}: Product) => {
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
        <Text>Energy {displayRound(energy.kcal)} kcal</Text>
        <Box direction="row">
          <Box><b>Carbohydrates</b></Box>
          <Box fill={true} alignSelf="end" align="end"><b>{displayRound(carbohydrates.total)} g</b></Box>
        </Box>
        <Box direction="row">
          <Box margin={{left: "large"}} >Sugar</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(carbohydrates.sugar)} g</Box>
        </Box>
        <Box direction="row">
          <Box fill margin={{left: "large"}} >Added Sugar</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(carbohydrates.addedSugar)} g</Box>
        </Box>
        <Box direction="row">
          <Box margin={{left: "large"}} >Fiber</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(carbohydrates.fiber)} g</Box>
        </Box>
        <Box direction="row">
          <Box margin={{left: "large"}} > Starch</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(carbohydrates.starch)} g</Box>
        </Box>
        <Box direction="row">
          <Box><b>Fat</b></Box>
          <Box fill={true} alignSelf="end" align="end"><b>{displayRound(fat.total)} g</b></Box>
        </Box>
        <Box direction="row">
          <Box margin={{left: "large"}} > Saturated</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(fat.saturated)} g</Box>
        </Box>
        <Box direction="row">
          <Box margin={{left: "large"}} > Monosaturated</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(fat.monounsaturated)} g</Box>
        </Box>
        <Box direction="row">
          <Box margin={{left: "large"}} > Trans</Box>
          <Box fill={true} alignSelf="end" align="end">{displayRound(fat.trans)} g</Box>
        </Box>
        <Box direction="row">
          <Box><b>Protein</b></Box>
          <Box fill={true} alignSelf="end" align="end"><b>{displayRound(protein.total)} g</b></Box>
        </Box>
        <Box direction="row">
          <Box><b>Salt</b></Box>
          <Box fill={true} alignSelf="end" align="end"><b>{displayRound(salt.total)} g</b></Box>
        </Box>
      </Box>
    </AccordionPanel>
  );
};

export default ConsumedItem;
