import { Nutrients, Product } from "../schema/product";
import { NutrientRow } from "../../components/NutrientTableRow";

export const calculatePer = (value: number, per: number, baseUnit: number) => {
  const one = value / 100;
  return one * (per * baseUnit);
};

export const displayRound = (value: number): number => {
  return +value.toFixed(3);
};

export interface RawNutrientRow {
  macronutrient: string;
  nutrient: string;
  amount: number;
}

const asColumn = (
  macronutrient: string,
  nutrient: string,
  amount: number
): RawNutrientRow => ({
  macronutrient,
  nutrient,
  amount,
});

const toRow = (
  level: number,
  name: string,
  highlight: boolean,
  amount?: number
): NutrientRow => {
  return {
    level,
    name,
    highlight,
    amount,
  };
};

export const calculateToDisplay = (
  value: number,
  per: number,
  baseUnit: number
) => {
  const result = calculatePer(value, per, baseUnit);
  return displayRound(result);
};

const nutrientsToTable = (nutrients: any, amount: number, baseUnit: number) => {
  const result: any[] = [];
  Object.keys(nutrients).forEach((key) => {
    if (key === "energy" || key === "vitamins") {
      return;
    }
    result.push(
      toRow(
        0,
        key,
        true,
        calculateToDisplay(nutrients[key].total, amount, baseUnit)
      )
    );
    result.push(...microNutrient(nutrients[key], amount, baseUnit, 1));
  });

  return result;
};

const microNutrient = (
  nutrient: any,
  amount: number,
  baseUnit: number,
  level: number
): any => {
  let rows: any[] = [];
  Object.keys(nutrient).forEach((key) => {
    if (key === "total") {
      return;
    }
    const n = nutrient[key];
    if (typeof n === "object" && n) {
      const next: any[] = microNutrient(n, amount, baseUnit, level + 1);
      rows.push(toRow(level, key, false));
      next.forEach((r) => rows.push(r));
    } else {
      rows.push(
        toRow(level, key, false, calculateToDisplay(n, amount, baseUnit))
      );
      // rows.push(asColumn(base, key, calculateToDisplay(n, amount, baseUnit)));
    }
  });

  return rows;
};

export const extractTabularNutrients = (
  nutrients: Nutrients,
  amount: number,
  baseUnit: number
): Array<NutrientRow> => {
  console.log(nutrients);
  return nutrientsToTable(nutrients, amount, baseUnit);
};
