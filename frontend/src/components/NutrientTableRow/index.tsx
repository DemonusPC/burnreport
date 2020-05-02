import React from 'react';
import styled from 'styled-components';
import { RawNutrientRow } from '../../util/data/calculations';
import { TableCell, TableRow, Text } from 'grommet';

interface NutrientTableRowProps {
    row: RawNutrientRow
}

const AmountCell = styled(TableCell)`
    text-align: right;
`;

const MacroAmountCell = styled(AmountCell)`
  font-weight: bold;
`;

const TotalRow = styled(TableRow)`
    border-top: 1px solid #000000;

    &:first-child {
        border-top: none;
    }
`;

const NutrientTableRow = ({row} : NutrientTableRowProps) => {
    // Nutrient table row groups by Macro Nutrient
    // Setting it as a main category
    if (row.nutrient === "total") {
        return (
          <TotalRow key={`${row.macronutrient}-${row.nutrient}`}>
            <TableCell>
              <Text weight="bold">{row.macronutrient}</Text>
            </TableCell>
            <TableCell />
  
            <MacroAmountCell>
              {row.amount} g
            </MacroAmountCell>
          </TotalRow>
        );
      }
      return (
        <TableRow key={`${row.macronutrient}-${row.nutrient}`}>
          <TableCell />
          <TableCell>
            <Text>{row.nutrient}</Text>
          </TableCell>
  
          <AmountCell>
              {row.amount} g
          </AmountCell>
        </TableRow>
      );
}

export default NutrientTableRow;