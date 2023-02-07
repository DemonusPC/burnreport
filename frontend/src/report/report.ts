import { Nutrients } from "../nutrients/nutrients";
import { emptyNutrients, Portion, Product } from "../product/product";

export type ReportResult = {
  timeDone: number;
  result: {
    total: Nutrients;
    consumed: Product[];
  };
};

export enum ProductEntity {
  Product = "Product",
  Spi = "Spi",
}
export interface ConsumedRaw {
  id: number;
  entity: ProductEntity;
  name: string;
  amount: number;
  unit: Portion;
  unitOptions: Array<Portion>;
}

export interface ReportItem {
  entity: ProductEntity;
  numericIdentifier: number;
  amount: number;
}

export interface Report {
  consumed: ReportItem[];
}

export const emptyReport = (): ReportResult => {
  return {
    timeDone: Date.now(),
    result: {
      total: emptyNutrients(),
      consumed: [],
    },
  };
};
