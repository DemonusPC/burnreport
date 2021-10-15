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
    unit: Unit.Grams,
  };

  return result;
};

export interface ProductAPIStatus {
  status: string;
  id?: number;
}

export interface ProductSize {
  id: number;
  product: number;
  name: string;
  grams: number;
}

export interface Portion {
  product: number;
  name: string;
  grams: number;
}
