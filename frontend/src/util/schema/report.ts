export interface ConsumedProduct {
    id: number;
    name: string;
    amount: number;
}
  
export interface Report {
    consumed: ConsumedProduct[]
}