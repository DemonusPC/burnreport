import { grommet } from "grommet/themes";

import { deepMerge } from "grommet/utils";

const BRAND = "#404345";
// const BRAND_DARK = "#1a1b1c" ;

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

export const burnReportTheme2 = {
  name: "my theme",
  rounding: 4,
  spacing: 24,
  defaultMode: "light",
  global: {
    colors: {
      brand: {
        dark: BRAND,
        light: BRAND,
      },
      background: {
        dark: "#111111",
        light: "#FFFFFF",
      },
      "background-back": {
        dark: "#111111",
        light: "#EEEEEE",
      },
      "background-front": {
        dark: "#222222",
        light: "#FFFFFF",
      },
      "background-contrast": {
        dark: "#FFFFFF11",
        light: "#11111111",
      },
      text: {
        dark: "#EEEEEE",
        light: "#333333",
      },
      "text-strong": {
        dark: "#FFFFFF",
        light: "#000000",
      },
      "text-weak": {
        dark: "#CCCCCC",
        light: "#444444",
      },
      "text-xweak": {
        dark: "#999999",
        light: "#666666",
      },
      border: {
        dark: "#444444",
        light: "#CCCCCC",
      },
      control: "brand",
      "active-background": "background-contrast",
      "active-text": "text-strong",
      "selected-background": "brand",
      "selected-text": "text-strong",
      "status-critical": "#FF4040",
      "status-warning": "#FFAA15",
      "status-ok": "#00C781",
      "status-unknown": "#CCCCCC",
      "status-disabled": "#CCCCCC",
      "graph-0": "brand",
      "graph-1": "status-warning",
    },
    font: {
      family: "Helvetica",
    },
    active: {
      background: "active-background",
      color: "active-text",
    },
    hover: {
      background: "active-background",
      color: "active-text",
    },
    selected: {
      background: "selected-background",
      color: "selected-text",
    },
  },
  chart: {},
  diagram: {
    line: {},
  },
  meter: {},
};
