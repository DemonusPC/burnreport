import { BarValue } from "../containers/Bar";
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

export const nutrientsToBarValues = (data: Nutrients): BarValue[] => {
  const result: BarValue[] = [];
  const { carbohydrates, fat, protein } = data;

  result.push({
    value: carbohydrates.total,
    label: "Carbohydrates",
    color: "#ffb347",
  });
  result.push({ value: fat.total, label: "fat", color: "#1e90ff" });
  result.push({ value: protein.total, label: "protein", color: "#dc143c" });

  return result;
};

export const nutrientsToBarTotal = (data: Nutrients): number => {
  return data.carbohydrates.total + data.fat.total + data.protein.total;
};

export const polyunsaturatedToBarValues = (data: PolyunsaturatedFat) => {
  const result = [];

  result.push({ value: data.omega3 || 0, label: "Omega 3", color: "#1eddff" });
  result.push({ value: data.omega6 || 0, label: "Omega 6", color: "#875b6f" });

  return result;
};

export const polyunsaturatedToBarTotal = (data: PolyunsaturatedFat) => {
  return (data.omega3 || 0) + (data.omega6 || 0);
};
