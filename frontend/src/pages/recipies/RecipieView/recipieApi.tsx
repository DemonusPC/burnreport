
import useSWR from "swr";
import { Recipie } from "../../../recipie/recipie";
import { fetcher } from "../../../util/data/requests";

export type GetRecipie = (recipieId: number) => { recipie: Recipie | null | undefined, error?: any };

export const fetchRecipie: GetRecipie = (recipieId: number) => {
    const { data, error } = useSWR<Recipie | null>(
        encodeURI(`/api/recipies/${recipieId}`),
        fetcher
    );

    return {
        recipie: data,
        error
    }

}