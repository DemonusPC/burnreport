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
    if (key === "energy") {
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

export const reportNutrientsToTable = (product: Product) => {
  if (!product) {
    return [];
  }
  const rows = [];

  const { carbohydrates, fat, protein, salt } = product.nutrients;

  rows.push(asColumn("Carbohydrates", "total", carbohydrates.total));
  rows.push(asColumn("Carbohydrates", "sugar", carbohydrates.sugar));
  rows.push(
    asColumn("Carbohydrates", "added sugar", carbohydrates.addedSugar || 0)
  );
  rows.push(asColumn("Carbohydrates", "fiber", carbohydrates.fiber || 0));
  rows.push(asColumn("Carbohydrates", "starch", carbohydrates.starch || 0));

  rows.push(asColumn("Fat", "total", fat.total));
  rows.push(asColumn("Fat", "saturated", fat.saturated));
  rows.push(asColumn("Fat", "trans", fat.trans || 0));

  rows.push(asColumn("Protein", "total", protein.total));

  rows.push(asColumn("Salt", "total", salt.total));

  return rows;
};
