import { Heading, Box } from "grommet";
import ProductSearchForm from "../../containers/ProductSearchForm";
import { Product } from "../../util/schema/product";
import useQuery from "../../util/useQuery";
import { ResultList } from "../../util/data/requests";
import useSWR from "swr";
import ProductCell from "../../components/ProductCell";

export const totalMacroInGrams = (product: Product) => {
  const carbs = product.carbohydrates.total;
  const fat = product.fat.total;
  const protein = product.protein.total;

  const total = carbs + fat + protein;

  return total;
};

const ProductsList = () => {
  const toSearch = useQuery().get("p") || "";
  const { data, error } = useSWR<ResultList<Product>>(
    encodeURI(`/api/search?p=${toSearch}`)
  );

  if (error) return <div>An error occured</div>;
  if (!data) return <div>loading...</div>;

  const productResult = data.result.map((p: Product) => {
    return <ProductCell {...p} />;
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
