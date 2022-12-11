import { Nutrients } from "../nutrients/nutrients";
import { Product } from "../product/product";


type Ingredient = {
    product: Product;
    amount: number;
}

export type Recipie = {
    id: number;
    name: string;
    ingredients: Array<Ingredient>
    total: Nutrients;
}