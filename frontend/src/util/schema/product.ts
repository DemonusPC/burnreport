import Vitamins from "./vitamins";

export interface Energy {
  kcal: number;
  kj: number;
}

export interface Carbohydrates {
  total: number;
  sugar: number;
  fiber?: number;
  addedSugar?: number;
  starch?: number;
}

export interface PolyunsaturatedFat {
  total: number;
  omega3?: number;
  omega6?: number;
}
export type MonoUnsaturatedFat = {
  total: number;
  omega7?: number;
  omega9: number;
};

export type UnsaturatedFat = {
  mono?: MonoUnsaturatedFat;
  poly?: PolyunsaturatedFat;
};

export interface Fat {
  total: number;
  saturated: number;
  unsaturated?: UnsaturatedFat;
  trans?: number;
}

export interface Protein {
  total: number;
}

export interface Salt {
  total: number;
}

export type Nutrients = {
  energy: Energy;
  carbohydrates: Carbohydrates;
  fat: Fat;
  protein: Protein;
  salt: Salt;
  vitamins?: Vitamins;
};

export enum Unit {
  Grams,
  Mililiters,
}
export type Product = {
  id: number;
  name: string;
  nutrients: Nutrients;
  unit: Unit;
};

export const emptyProduct = (): Product => {
  const result: Product = {
    id: 0,
    name: "",
    nutrients: {
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
    },
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
