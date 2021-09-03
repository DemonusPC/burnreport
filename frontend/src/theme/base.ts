import { grommet } from "grommet/themes";

import { deepMerge } from "grommet/utils";

const BRAND = "#404345";

export const burnReportTheme = deepMerge(grommet, {
  global: {
    colors: {
      brand: {
        dark: "#f8f8f8",
        light: BRAND,
      },
      background: {
        dark: "#2b2b2b",
        light: "#FFFFFF",
      },
      "accent-1": {
        dark: "#f8f8f8",
        light: "#b62203",
      },
      "accent-3": "#b62203",
      focus: "#b62203",
    },
    focus: {
      shadow: {
        color: "#b62203",
      },
    },
  },
  button: {
    border: {
      radius: "4px",
    },
  },
});
