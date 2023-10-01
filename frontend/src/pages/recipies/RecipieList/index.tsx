import React from 'react'

import { Heading, Box } from "grommet";
import useQuery from '../../../util/useQuery';
import ProductSearchForm, { SearchSuggestion } from '../../../containers/ProductSearchForm';
import RecipieCell from '../../../components/RecipieCell';
import { getRecipieSearchSuggestions, recipieListUrl } from '../Search';
import { UseRecipieSearch, useRecipieSearch } from '../recipieAPi/recipieApi';

type RecipieListProps = {
    useRecipie?: UseRecipieSearch
}

const RecipieList = ({ useRecipie = useRecipieSearch }: RecipieListProps) => {
    const toSearch = useQuery().get("p") || "";

    const { recipie, error } = useRecipie(toSearch);

    if (error) return <div>An error occured</div>;
    if (!recipie) return <div>loading...</div>;

    const productResult = recipie.map((p: SearchSuggestion) => {
        return <RecipieCell {...p} key={`${p.text}${p.subText}`} />;
    });

    return (
        <Box pad="large" gridArea="main">
            <Box>
                <Heading>Recipie Search</Heading>
            </Box>
            <Box
                pad={{
                    vertical: "medium",
                }}
                width="large"
            >
                <ProductSearchForm getSuggestions={getRecipieSearchSuggestions} getSubmitUrl={recipieListUrl} />
            </Box>

            {productResult}
        </Box>
    );
};

export default RecipieList;