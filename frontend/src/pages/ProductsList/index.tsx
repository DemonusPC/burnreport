import { Heading, Box } from "grommet";
import ProductSearchForm, {
  SearchSuggestion,
} from "../../containers/ProductSearchForm";
import useQuery from "../../util/useQuery";
import { fetcher, ResultList } from "../../util/data/requests";
import useSWR from "swr";
import ProductCell from "../../components/ProductCell";

const ProductsList = () => {
  const toSearch = useQuery().get("p") || "";
  const { data, error } = useSWR<ResultList<SearchSuggestion>>(
    encodeURI(`/api/search?p=${toSearch}&e=product`),
    fetcher
  );

  if (error) return <div>An error occured</div>;
  if (!data) return <div>loading...</div>;

  const productResult = data.result.map((p: SearchSuggestion) => {
    return <ProductCell {...p} key={`${p.text}${p.subText}`} />;
  });

  return (
    <Box pad="large" gridArea="main">
      <Box>
        <Heading>Product Search</Heading>
      </Box>
      <Box
        pad={{
          vertical: "medium",
        }}
        width="large"
      >
        <ProductSearchForm initialText={toSearch} />
      </Box>

      {productResult}
    </Box>
  );
};

export default ProductsList;
