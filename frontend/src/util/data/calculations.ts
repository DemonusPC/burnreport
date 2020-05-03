import { Product } from "../schema/product";
import { FlatProduct } from "../schema/report";

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

export const reportNutrientsToTable = (product: Product) => {
    if(!product) {
        return [];
    }
    const rows = [];
    
    rows.push(asColumn('Carbohydrates', 'total', product.carbohydrates.total));
    rows.push(asColumn('Carbohydrates', 'sugar', product.carbohydrates.sugar));
    rows.push(asColumn('Carbohydrates', 'added sugar', product.carbohydrates.addedSugar));
    rows.push(asColumn('Carbohydrates', 'fiber', product.carbohydrates.fiber));
    rows.push(asColumn('Carbohydrates', 'starch', product.carbohydrates.starch));

    rows.push(asColumn('Fat', 'total', product.fat.total));
    rows.push(asColumn('Fat', 'saturated', product.fat.saturated));
    rows.push(asColumn('Fat', 'monounsaturated', product.fat.monounsaturated));
    rows.push(asColumn('Fat', 'trans', product.fat.trans));

    rows.push(asColumn('Protein', 'total', product.protein.total));

    rows.push(asColumn('Salt', 'total', product.salt.total));


    return rows;
}

export const flattenProductList = (products: Product[]): Array<FlatProduct> => {
    const result: Array<FlatProduct> = products.map(product => {
        return {
            name: product.name,
            manufacturer: product.manufacturer,
            kcal: product.energy.kcal,
            kj: product.energy.kj,
            carbohydrates: product.carbohydrates.total,
            sugar: product.carbohydrates.sugar,
            addedSugaer: product.carbohydrates.addedSugar,
            fiber: product.carbohydrates.fiber,
            starch: product.carbohydrates.starch,
            fat: product.fat.total,
            saturated: product.fat.saturated,
            monounsaturated: product.fat.monounsaturated,
            trans: product.fat.trans,
            protein: product.protein.total,
            salt: product.salt.total
        }
    });

    return result;
}
