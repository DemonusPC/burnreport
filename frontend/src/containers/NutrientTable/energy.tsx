import React from 'react';
import { Energy } from '../../nutrients/nutrients';
import { calculateToDisplay } from '../../util/data/calculations';
import { Box, Text } from 'grommet';

type EneryRowProps = {
    energy: Energy,
    amount: number,
    baseUnit: number
}

const EnergyRow = ({ energy, amount = 100, baseUnit }: EneryRowProps) => {

    const kj = calculateToDisplay(energy.kj, amount, baseUnit);
    const kcal = calculateToDisplay(energy.kcal, amount, baseUnit);

    return <>
        <Box
            direction="row"
            margin={{
                top: "medium",
            }}
            justify='between'
            border={
                {
                    color: "border",
                    size: "1px",
                    style: "solid",
                    side: "top",
                }

            }
            pad={{ top: "medium" }}
        >
            <Box margin={{ left: "0" }}>
                <Text weight={"bold"}>Energy </Text>
            </Box>
            <Box>
                <Text weight={"bold"}>
                    {kj} kJ / {kcal} kcal
                </Text>
            </Box>
        </Box>

    </>
};

export default EnergyRow;