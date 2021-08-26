import React from "react";
import { Meter } from "grommet";

export interface OmegaBarProps {
    omega3: number,
    omega6: number,
}

const mapToValues = (omega3: number, omega6: number) => {
    const result = [];

    result.push({value: omega3, label: "Omega 3", color: "#1eddff"});
    result.push({value: omega6, label: "Omega 6", color: "#875b6f"});

    return result;

}

const OmegaBar = ({omega3, omega6}: OmegaBarProps) => {
  const values = mapToValues(omega3,omega6);

  const total = omega3 + omega6;

  // Base height is 24px
  return (
    <Meter
    size="full"
      values={values}
      max={total}
      aria-label="meter"
    />
  );
};

OmegaBar.defaultProps = {
    omega3: 0,
    omega6: 0
}

export default OmegaBar;
