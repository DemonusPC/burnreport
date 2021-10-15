import {
  Product,
  ProductAPIStatus,
  ProductSize,
  Portion,
} from "../../product/product";
import { Report } from "../schema/report";
import { ReportResult } from "../../containers/ReportRender";
import { SearchSuggestion } from "../../containers/ProductSearchForm";

export interface RestResult<T> {
  status: boolean;
  data?: T;
}

export interface ResultList<T> {
  result: Array<T>;
}

export const fetcher = (url: string) =>
  fetch(url).then((r) => {
    if (!r.ok) {
      throw new Error("An error occured");
    }
    return r.json();
  });

export const getProductSearchSuggestions = async (
  suggestion: string
): Promise<Array<SearchSuggestion>> => {
  const request = await fetch(
    encodeURI(`/api/search/suggestions?p=${suggestion}`)
  );

  const result: ResultList<SearchSuggestion> = await request.json();

  return result.result;
};

const generatePostRequest = <T, O>(uri: string): ((data: T) => Promise<O>) => {
  return async (data: T): Promise<O> => {
    const response = await fetch(uri, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    });

    const result: O = await response.json();

    return result;
  };
};

// This is different from a generator
const deleteRequest = async <O>(uri: string): Promise<O> => {
  const response = await fetch(uri, {
    method: "DELETE",
  });

  const result: O = await response.json();

  return result;
};

export const postReport = generatePostRequest<Report, ReportResult>(
  `/api/report`
);

export const postProduct = generatePostRequest<Product, ProductAPIStatus>(
  `/api/products`
);

export const deleteProduct = async (id: number): Promise<ProductAPIStatus> => {
  return await deleteRequest(`/api/products/${id}`);
};

export const postCSVProducts = async (data: any): Promise<ProductAPIStatus> => {
  const response = await fetch(`/api/products/csv`, {
    method: "POST",
    body: data,
  });

  const result: ProductAPIStatus = await response.json();

  return result;
};

export const getProductSizesById = async (id: number) => {
  const request = await fetch(encodeURI(`/api/products/${id}/portions`));
  const result: ResultList<ProductSize> = await request.json();

  return result.result;
};

export const postPortions = generatePostRequest<
  Portion[],
  RestResult<ProductAPIStatus>
>(`/api/products/portions`);

export const deletePortion = async (
  id: number,
  name: string
): Promise<ProductAPIStatus> => {
  return await deleteRequest<ProductAPIStatus>(
    `/api/products/${id}/portions/${name}`
  );
};
