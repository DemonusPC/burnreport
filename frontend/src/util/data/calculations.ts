import { Product } from "../schema/product";
import { FlatProduct } from "../schema/report";

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

const calculateToDisplay = (value: number, per: number, baseUnit: number) => {
  const result = calculatePer(value, per, baseUnit);
  return displayRound(result);
};

export const extractTabularNutrients = (
  product: Product,
  amount: number,
  baseUnit: number
): Array<RawNutrientRow> => {
  if (!product) {
    return [];
  }
  const rows = [];

  const { carbohydrates, fat, protein, salt } = product.nutrition;
  rows.push(
    asColumn(
      "Carbohydrates",
      "total",
      calculateToDisplay(carbohydrates.total, amount, baseUnit)
    )
  );
  rows.push(
    asColumn(
      "Carbohydrates",
      "sugar",
      calculateToDisplay(carbohydrates.sugar, amount, baseUnit)
    )
  );
  rows.push(
    asColumn(
      "Carbohydrates",
      "added sugar",
      calculateToDisplay(carbohydrates.addedSugar || 0, amount, baseUnit)
    )
  );
  rows.push(
    asColumn(
      "Carbohydrates",
      "fiber",
      calculateToDisplay(carbohydrates.fiber || 0, amount, baseUnit)
    )
  );
  rows.push(
    asColumn(
      "Carbohydrates",
      "starch",
      calculateToDisplay(carbohydrates.starch || 0, amount, baseUnit)
    )
  );

  rows.push(
    asColumn("Fat", "total", calculateToDisplay(fat.total, amount, baseUnit))
  );
  rows.push(
    asColumn(
      "Fat",
      "saturated",
      calculateToDisplay(fat.saturated, amount, baseUnit)
    )
  );
  rows.push(
    asColumn(
      "Fat",
      "trans",
      calculateToDisplay(fat.trans || 0, amount, baseUnit)
    )
  );

  rows.push(
    asColumn(
      "Protein",
      "total",
      calculateToDisplay(protein.total, amount, baseUnit)
    )
  );

  rows.push(
    asColumn("Salt", "total", calculateToDisplay(salt.total, amount, baseUnit))
  );

  return rows;
};

export const reportNutrientsToTable = (product: Product) => {
  if (!product) {
    return [];
  }
  const rows = [];

  const { carbohydrates, fat, protein, salt } = product.nutrition;

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
