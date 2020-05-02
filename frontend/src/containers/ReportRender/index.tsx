import React from 'react'; 
import { Product } from '../../util/schema/product';
import { Box, Heading, Table, TableHeader, TableRow, TableCell, TableBody, Text } from 'grommet';

export interface ReportResult {
    timeDone: number,
    result: {
        total: {
            kcal: number,
            carbohydrates: number,
            fat: number,
            protein: number
        }
        consumed: Product[]
    }

}


const mapColumns = (columns: Array<string>) => {
    return columns.map((col) => (
        <TableCell key={col} scope="col">
        <Text>{col}</Text>
      </TableCell> 
    ))
}

const mapRows = (consumed: Product[]) => {
    return consumed.map((product) => (
        <TableRow key={product.id}>
        <TableCell>
          <Text>{product.name}</Text>
        </TableCell>
        <TableCell>
          <Text>{product.energy.kcal}</Text>
        </TableCell>
        <TableCell>
          <Text>{product.carbohydrates.total}</Text>
        </TableCell>
        <TableCell>
          <Text>{product.fat.total}</Text>
        </TableCell>
        <TableCell>
          <Text>{product.protein.total}</Text>
        </TableCell>
      </TableRow>
    ))
}

const columns = ["Name", "Energy (kcal)", "Carbohydrates", "Fat", "Protein" ]; 

const ReportRender = ({result} : ReportResult) => {
    return (
        <Box>
            <Heading>Report</Heading>
    <Heading level={2}>Total Kcal: {result.total.kcal}</Heading>
        <Box>
        <Table alignSelf="stretch">
          <TableHeader>
            <TableRow>
            {mapColumns(columns)}
            </TableRow>
          </TableHeader>
          <TableBody>
              {mapRows(result.consumed)}
          </TableBody>
        </Table>
        </Box>
        </Box>
    )
}

ReportRender.defaultProps = {
    timeDone: Date.now(),
    result: {
        total: {
            kcal: 0,
        },
        consumed: []
    }
}

export default ReportRender;