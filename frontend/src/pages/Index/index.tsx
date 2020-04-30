import React from "react";
import {
  Heading,
  Button,
  Box,
  Table,
  TableHeader,
  TableRow,
  TableCell,
  Text,
  TableBody,
} from "grommet";
import SearchForm from "../../containers/SearchForm";
import { Product } from "../../util/data/product";
import ProductItem from "../../containers/ProductItem";
import ReportRender, { ReportResult } from "../../containers/ReportRender";

interface ConsumedProduct {
  id: number;
  name: string;
  amount: number;
}

interface Report {
  consumed: ConsumedProduct[]
}

const emptyReport = (): ReportResult => {
  return {
    timeDone: Date.now(),
    result: {
        total: {
            kcal: 0,
            carbohydrates: 0,
            fat: 0,
            protein: 0
        },
        consumed: []
    }
  }
}

const postReport = async (report: Report): Promise<ReportResult> => {
    // Default options are marked with *

    const response = await fetch("http://localhost:8080/report", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        origin: "*",
      },
      mode: "cors",
      body: JSON.stringify(report)
    });

    
    console.log(response);

    const json = await response.json();
    console.log(json);

    return json;
}

const sendReport = (consumed: ConsumedProduct[], setReport: any) => {
  const report : Report = {
    consumed
  }; 
  
  postReport(report).then((json) => {
    setReport({completed: true, report: json});
  });
  
  // console.log(report);
}

const fetchProducts = async (suggestion: string) => {
  const request = await fetch(
    encodeURI(`http://localhost:8080/search?p=${suggestion}`),
    {
      headers: {
        origin: "localhost",
      },
    }
  );
  const json = await request.json();
  return json;
};

const emptyConsumed = (): Array<ConsumedProduct> => {
  return [];
};

const mapProductItems = (
  products: ConsumedProduct[],
  values: ConsumedProduct[],
  setFunction: any
) => {
  return products.map((product) => (
    <ProductItem
      key={product.id}
      id={product.id}
      name={product.name}
      amount={product.amount}
      changeFunc={(event) => {
        const result = values.map((pp) => {
          if (pp.id === product.id) {
            return {
              id: pp.id,
              name: pp.name,
              amount: +event.target.value,
            };
          }
          return pp;
        });
        setFunction(result);
      }}
      deleteFunc={() => {
        const result = values.filter((p) => p.id !== product.id);
        setFunction(result);
      }}
    />
  ));
};

const Index = () => {
  const [consumed, setConsumed] = React.useState(emptyConsumed());
  const [report, setReport] = React.useState({
    completed: false,
    report: emptyReport()
  })

  const addElement = (product: Product) => {
    if(consumed.some(c => c.id === product.id)){
      return;
    }
    const result = consumed.concat([
      { id: product.id, name: product.name, amount: 100 },
    ]);
    setConsumed(result);
  };

  const submit = () => {
    sendReport(consumed, setReport);
  }

  const reset = () => {
    setConsumed(emptyConsumed());
  }

 

  return (
    <>
    {!report.completed ?
    <Box pad="large">
      <Heading>Create Report</Heading>
      <Box pad={{ bottom: "large" }}>
        <Table>
          <TableHeader>
            <TableRow>
              <TableCell key={"name"} scope="col">
                <Text>Name</Text>
              </TableCell>
              <TableCell key={"amount"} scope="col">
                <Text>amount (g)</Text>
              </TableCell>
              <TableCell key={"delete"} scope="col">
                <Text></Text>
              </TableCell>
            </TableRow>
          </TableHeader>
          <TableBody>
            {mapProductItems(consumed, consumed, setConsumed)}
          </TableBody>
        </Table>
      </Box>

      <Box pad={{ bottom: "large" }}>
        <SearchForm
          selectedFunction={(product: Product) => {
            addElement(product);
          }}
          suggestFunction={fetchProducts}
        />
      </Box>

      <Box>
        <Box direction="row" gap="medium">
          <Button type="submit" primary label="Submit" onClick={submit} />
          <Button type="reset" label="Reset" onClick={reset} />
        </Box>
      </Box>
    </Box>
    :
    <Box pad="large">
      <ReportRender result={report.report.result} />
    </Box>
    }
    </>
  );
};

export default Index;
