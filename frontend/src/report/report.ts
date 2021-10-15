import { ReportResult } from "../containers/ReportRender";
import { emptyNutrients, Portion } from "../product/product";

export interface ConsumedRaw {
  id: number;
  name: string;
  amount: number;
  unit: Portion;
  unitOptions: Array<Portion>;
}

export interface ConsumedProduct {
  id: number;
  name: string;
  amount: number;
}

export interface Report {
  consumed: ConsumedProduct[];
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
