// TODO: The search api needs to be refactored better. For now duplication is fine

import useSWR from "swr";
import { SearchSuggestion } from "../../../containers/ProductSearchForm";
import { ResultList, fetcher } from "../../../util/data/requests";


export type UseRecipieSearch = (queryText: string) => { recipie: SearchSuggestion[] | null | undefined, error?: any };


export const useRecipieSearch: UseRecipieSearch = (queryText: string) => {
    const { data, error } = useSWR<ResultList<SearchSuggestion> | null>(
        encodeURI(`/api/search?p=${queryText}&e=recipie`),
        fetcher
    );

    if (!data) {
        return {
            recipie: data,
            error
        }
    }

    return {
        recipie: data.result,
        error
    }

}
