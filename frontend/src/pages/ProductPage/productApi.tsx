import useSWR from "swr";
import { Portion, Product } from "../../product/product";
import { ResultList, fetcher } from "../../util/data/requests";

export type GetProduct = (productId: number) => { data: Product | null | undefined, error?: any };

export type GetProductPortions = (productId: number) => { portions: Portion[] | null | undefined, error?: any };


export const fetchProduct: GetProduct = (productId: number) => {
    const { data, error } = useSWR<Product | null>(
        encodeURI(`/api/products/${productId}`),
        fetcher
    );

    return {
        data,
        error
    }

}

const base: Portion = {
    id: 0,
    product: 0,
    name: "grams",
    grams: 1,
};

export const fetchProductPortions: GetProductPortions = (productId: number) => {
    const { data, error } = useSWR<ResultList<Portion>>(
        encodeURI(`/api/products/${productId}/portions`),
        fetcher
    );

    if (!data) {
        return {
            portions: data,
            error
        }
    }

    const portions = [base].concat(data.result);

    return {
        portions,
        error
    }

}