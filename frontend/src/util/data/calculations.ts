import { Product } from "../schema/product";

export const calculatePer = (value: number, per: number): number => {
    const one = value / 100;
    return one * per;
}


export const displayRound = (value: number): number => {
    return +value.toFixed(3);
}

export interface RawNutrientRow {
    macronutrient: string,
    nutrient: string,
    amount: number
}

const asColumn = (macronutrient: string, nutrient: string, amount: number): RawNutrientRow => ({
    macronutrient,
    nutrient,
    amount
})

const calculateToDisplay = (value: number, per: number) => {
    const result = calculatePer(value, per);
    return displayRound(result);
}

export const extractTabularNutrients = (product: Product, amount: number): Array<RawNutrientRow> => {

    if(!product) {
        return [];
    }
    const rows = [];
    
    rows.push(asColumn('Carbohydrates', 'total', calculateToDisplay(product.carbohydrates.total, amount)));
    rows.push(asColumn('Carbohydrates', 'sugar', calculateToDisplay(product.carbohydrates.sugar, amount)));
    rows.push(asColumn('Carbohydrates', 'added sugar', calculateToDisplay(product.carbohydrates.addedSugar, amount)));
    rows.push(asColumn('Carbohydrates', 'fiber', calculateToDisplay(product.carbohydrates.fiber, amount)));
    rows.push(asColumn('Carbohydrates', 'starch', calculateToDisplay(product.carbohydrates.starch, amount)));

    rows.push(asColumn('Fat', 'total', calculateToDisplay(product.fat.total, amount)));
    rows.push(asColumn('Fat', 'saturated', calculateToDisplay(product.fat.saturated, amount)));
    rows.push(asColumn('Fat', 'monounsaturated', calculateToDisplay(product.fat.monounsaturated, amount)));
    rows.push(asColumn('Fat', 'trans', calculateToDisplay(product.fat.trans, amount)));

    rows.push(asColumn('Protein', 'total', calculateToDisplay(product.protein.total, amount)));

    rows.push(asColumn('Salt', 'total', calculateToDisplay(product.salt.total, amount)));


    return rows;
}