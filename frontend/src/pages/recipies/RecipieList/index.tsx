import React from 'react'



import { Heading, Box } from "grommet";
import useSWR from "swr";
import useQuery from '../../../util/useQuery';
import { fetcher, ResultList } from '../../../util/data/requests';
import ProductSearchForm, { SearchSuggestion } from '../../../containers/ProductSearchForm';
import RecipieCell from '../../../components/RecipieCell';
import { getRecipieSearchSuggestions, recipieListUrl } from '../Search';

const RecipieList = () => {
    const toSearch = useQuery().get("p") || "";
    const { data, error } = useSWR<ResultList<SearchSuggestion>>(
        encodeURI(`/api/search?p=${toSearch}&e=recipie`),
        fetcher
    );

    if (error) return <div>An error occured</div>;
    if (!data) return <div>loading...</div>;

    const productResult = data.result.map((p: SearchSuggestion) => {
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