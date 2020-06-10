import React from 'react';
import styled from 'styled-components';
import { Box, Heading, TableHeader, Table, Text, TableRow, TableCell, TableBody, Button } from 'grommet';
import SearchForm from '../SearchForm';
import { Product } from '../../util/schema/product';
import { getProductSearch, postReport } from '../../util/data/requests';
import { ConsumedProduct, Report } from '../../util/schema/report';
import ProductItem from '../ProductItem';


const emptyConsumed = (): Array<ConsumedProduct> => {
  return [];
};

const StyledTable = styled(Table)`
  min-width: 40%;
`;

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

const sendReport = (consumed: ConsumedProduct[], setReport: any) => {
  const report : Report = {
    consumed
  }; 
  
  postReport(report).then((json) => {
    setReport({completed: true, report: json});
  }); 
}

interface ReportFormProps {
  setReportFunction: any;
}


const ReportForm = ({setReportFunction} : ReportFormProps)  => {

  const [consumed, setConsumed] = React.useState(emptyConsumed());

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
    sendReport(consumed, setReportFunction);
  }
  
  const reset = () => {
    setConsumed(emptyConsumed());
  }
   
    return (
        <Box>
        <Heading>Create Report</Heading>
        <Box pad={{ bottom: "large" }}>
          <StyledTable>
            <TableHeader>
              <TableRow>
                <TableCell key={"name"} scope="col">
                  <Text>Name</Text>
                </TableCell>
                <TableCell key={"amount"} scope="col">
                  <Text>Amount (g)</Text>
                </TableCell>
                <TableCell key={"delete"} scope="col">
                  <Text></Text>
                </TableCell>
              </TableRow>
            </TableHeader>
            <TableBody>
              {mapProductItems(consumed, consumed, setConsumed)}
            </TableBody>
          </StyledTable>
        </Box>
  
        <Box pad={{ bottom: "large" }}>
          <SearchForm
            selectedFunction={(product: Product) => {
              addElement(product);
            }}
            suggestFunction={getProductSearch}
          />
        </Box>
  
        <Box>
          <Box direction="row" gap="medium">
            <Button type="submit" primary label="Submit" onClick={submit} />
            <Button type="reset" label="Reset" onClick={reset} />
          </Box>
        </Box>
      </Box>
    )
}

export default ReportForm;