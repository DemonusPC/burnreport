import useSWR from "swr";
import { Portion, Product } from "../../product/product";
import { ResultList, fetcher } from "../../util/data/requests";
import { SearchEntity, SearchSuggestion } from "../../containers/ProductSearchForm";

export type UseProduct = (productId: number) => { data: Product | null | undefined, error?: any };

export type UseProductPortions = (productId: number) => { portions: Portion[] | null | undefined, error?: any };

export type GetSearch = (suggestion: string) => Promise<SearchSuggestion[]>;

export const getProductSearchSuggestions: GetSearch = async (
    suggestion: string
): Promise<SearchSuggestion[]> => {
    const request = await fetch(encodeURI(`/api/search?p=${suggestion}`));

    const result: ResultList<SearchSuggestion> = await request.json();

    return result.result.filter((se) => se.entity !== SearchEntity.Recipie);
};

export const useProduct: UseProduct = (productId: number) => {
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

export const useProductPortions: UseProductPortions = (productId: number) => {
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