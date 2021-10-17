import { Meter } from "grommet";

export type BarValue = {
  value: number;
  label: string;
  color: string;
};
export type BarProps<T> = {
  data: T;
  mapToBarValues: (data: T) => BarValue[];
  calculateTotal: (data: T) => number;
};

const Bar = <T,>({
  data,
  mapToBarValues,
  calculateTotal,
}: BarProps<T>): JSX.Element => {
  const values = mapToBarValues(data);
  const total = calculateTotal(data);
  return <Meter size="full" values={values} max={total} aria-label="meter" />;
};

export default Bar;
