import React from "react";

import { Grid, ResponsiveContext } from "grommet";

const columns: any = {
  small: ["auto"],
  medium: ["20%", "60%", "20%"],
  large: ["25%", "50%", "25%"],
};

const rows = ["flex"];

// Set the different areas you need for every size

const fixedGridAreas: any = {
  small: [{ name: "main", start: [0, 0], end: [0, 0] }],
  medium: [{ name: "main", start: [1, 0], end: [1, 0] }],
  large: [{ name: "main", start: [1, 0], end: [1, 0] }],
};

const ResponsiveGrid = ({
  children,
  ...props
}: {
  children: any;
  props?: any;
}) => (
  <ResponsiveContext.Consumer>
    {(size) => {
      return (
        <Grid
          {...props}
          areas={fixedGridAreas[size] || fixedGridAreas.small}
          rows={rows}
          columns={columns[size] || columns.small}
        >
          {children}
        </Grid>
      );
    }}
  </ResponsiveContext.Consumer>
);

export default ResponsiveGrid;
