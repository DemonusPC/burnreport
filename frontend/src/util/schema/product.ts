export interface Energy {
    kcal: number,
    kj: number
}

export interface Carbohydrates {
    total: number,
    fiber: number,
    sugar: number,
    addedSugar: number,
    starch: number
}

export interface Fat {
    total: number,
    saturated: number,
    monounsaturated: number,
    trans: number
}

export interface Protein {
    total: number
}

export interface Salt {
    total: number
}

export interface Product {
    id: number,
    name: string,
    manufacturer: string,
    energy: Energy,
    carbohydrates: Carbohydrates,
    fat: Fat,
    protein: Protein,
    salt: Salt
}

export const emptyProduct = () : Product => {
    const result : Product = {
        id: 0,
        name: "",
        manufacturer: "",
        energy: {
            kcal: 0,
            kj: 0
        },
        carbohydrates: {
            total: 0,
            fiber: 0,
            sugar: 0,
            addedSugar: 0,
            starch: 0
        },
        fat: {
            total: 0,
            saturated: 0,
            monounsaturated: 0,
            trans: 0           
        },
        protein: {
            total: 0
        },
        salt : {
            total: 0
        }
    }

    return result;
} 

export interface ProductAPIStatus {
    status: string,
    id?: number 
}

export interface ProductSize {
    id: number,
    product: number,
    name: string,
    grams: number
}