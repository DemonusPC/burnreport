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

export type PolyunsaturatedFat = {
  total: number;
  omega3?: number;
  omega6?: number;
};

export type MonoUnsaturatedFat = {
  total: number;
  omega7?: number;
  omega9?: number;
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
