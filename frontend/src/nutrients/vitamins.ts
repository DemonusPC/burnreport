import { RowProps } from "../components/Row";

export default interface Vitamins {
  fat: {
    a: number;
    d: number;
    e: number;
    k: number;
  };
  water: {
    b1: number;
    b2: number;
    b3: number;
    b5: number;
    b6: number;
    b7: number;
    b9: number;
    b12: number;
    c: number;
  };
}

export const emptyVitamins = (): Vitamins => ({
  fat: {
    a: 9,
    d: 9,
    e: 0,
    k: 0,
  },
  water: {
    b1: 0,
    b2: 0,
    b3: 0,
    b5: 0,
    b6: 0,
    b7: 0,
    b9: 0,
    b12: 0,
    c: 0,
  },
});

export const vitaminsToRow = (entity: Vitamins): Array<RowProps> => {
  const result = [];

  const { fat, water } = entity;

  let k: keyof typeof fat;

  for (k in fat) {
    const v: number = fat[k];
    result.push({ entity: k, amount: v, unit: "mg" });
  }

  let c: keyof typeof water;

  for (c in water) {
    const v: number = water[c];
    result.push({ entity: c, amount: v, unit: "mg" });
  }

  return result;
};
