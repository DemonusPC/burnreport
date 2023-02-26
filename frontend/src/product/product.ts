import { Nutrients } from "../nutrients/nutrients";

export enum Unit {
  Grams = "Grams",
  Mililiters = "Mililiters",
}
export type Product = {
  id: number;
  name: string;
  nutrients: Nutrients;
  unit: Unit;
  spi: number | undefined;
};

export const emptyNutrients = (): Nutrients => {
  return {
    energy: {
      kcal: 0,
      kj: 0,
    },
    carbohydrates: {
      total: 0,
      sugar: 0,
    },
    fat: {
      total: 0,
      saturated: 0,
      trans: 0,
    },
    protein: {
      total: 0,
    },
    salt: {
      total: 0,
    },
  };
};

export const emptyProduct = (): Product => {
  const result: Product = {
    id: 0,
    name: "",
    nutrients: emptyNutrients(),
    spi: undefined,
    unit: Unit.Grams,
  };

  return result;
};

export interface ProductAPIStatus {
  status: string;
  id?: number;
}

export type Portion = {
  id?: number;
  product: number;
  name: string;
  grams: number;
};
