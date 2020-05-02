import React from "react";
import { Meter } from "grommet";

export interface NutrientBarProps {
    carbohydrates: number,
    fat: number,
    protein: number,
    total: number
}

const mapToValues = (carbohydrates: number, fat: number, protein: number) => {
    const result = [];

    result.push({value: carbohydrates, label: "Carbohydrates", color: "#ffb347"});
    result.push({value: fat, label: "fat", color: "#1e90ff"});
    result.push({value: protein, label: "protein", color: "#dc143c"});

    return result;

}

const NutrientBar = ({carbohydrates, fat, protein, total}: NutrientBarProps) => {
  const values = mapToValues(carbohydrates,fat,protein);


  // Base height is 24px
  return (
    <Meter
    size="full"
      values={values}
      max={total}
      aria-label="meter"
    />
  );
};

NutrientBar.defaultProps = {
    carbohydrates: 0,
    fat: 0,
    protein: 0,
    total: 0
}

export default NutrientBar;
