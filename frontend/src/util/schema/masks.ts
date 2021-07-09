// Masks for the masked input

export const standardMask = [
  {
    regexp: /^\d+$/,
    placeholder: "0",
  },
  { fixed: "." },
  {
    length: 2,
    regexp: /^[0-9]{1,4}$/,
    placeholder: "00",
  },
];

export const highPrecisionMask = [
  {
    regexp: /^\d+$/,
    placeholder: "0",
  },
  { fixed: "." },
  {
    length: 3,
    regexp: /^[0-9]{1,4}$/,
    placeholder: "000",
  },
];
