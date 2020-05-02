import { Product } from "../schema/product";
import { Report } from "../schema/report";
import { ReportResult } from "../../containers/ReportRender";

export const getProductSearch = async (suggestion: string): Promise<Array<Product>> => {
    const request =  await fetch(encodeURI(`/search?p=${suggestion}`));

    const result : Array<Product> = await request.json();

    return result;
}


export const getSingleProductById = async (id: number) => {
    const request =  await fetch(encodeURI(`/products/${id}`));

    const result : Array<Product> = await request.json();

    return result;
}
    
export const postReport = async (report: Report): Promise<ReportResult> => {
    const response = await fetch(`/report`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        origin: "*",
      },
      mode: "cors",
      body: JSON.stringify(report)
    });

    const result: ReportResult = await response.json();

    return result;
}