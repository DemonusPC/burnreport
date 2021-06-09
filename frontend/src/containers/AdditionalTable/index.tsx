import React from "react";
import Row, { RowProps } from "../../components/Row";

interface AdditionalTableProps<T> {
  entity?: T,
  mapper: (entity: T) => Array<RowProps>;
  unit: string;
}


const mapExtractedRows = (rows: Array<RowProps>) => {
  return rows.map((r) => (
    <Row  key={r.entity} amount={r.amount} entity={r.entity} unit={r.unit} />
  ));
};

const AdditionalTable = <T,>({ entity, mapper, unit }: AdditionalTableProps<T>) => {
  if(!entity) {
    return <></>
  }

  const rows = mapper(entity);

  return (
      <>
        {mapExtractedRows(rows)}
      </>
  );
};


export default AdditionalTable;
