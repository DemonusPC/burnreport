import { Product } from "./schema/product";

const data: Array<Product> = [
  {
    id: 12,
    name: "Sainsbury’s Spaghetti",
    manufacturer: "Sainsbury’s",
    energy: {
      kcal: 159.0,
      kj: 675.0,
    },
    carbohydrates: {
      total: 32.3,
      fiber: 0.0,
      sugar: 1.6,
      addedSugar: 0.0,
      starch: 30.7,
    },
    fat: {
      total: 0.7,
      saturated: 0.1,
      monounsaturated: 0.0,
      trans: 0.0,
    },
    protein: {
      total: 5.2,
    },
    salt: {
      total: 0.01,
    },
  },
  {
    id: 4,
    name: "Sainsbury’s Greek Style Natural Yogurt",
    manufacturer: "Sainsbury’s",
    energy: {
      kcal: 120.0,
      kj: 500.0,
    },
    carbohydrates: {
      total: 5.3,
      fiber: 0.0,
      sugar: 5.1,
      addedSugar: 0.0,
      starch: 0.0,
    },
    fat: {
      total: 9.2,
      saturated: 6.0,
      monounsaturated: 2.3,
      trans: 0.0,
    },
    protein: {
      total: 4.1,
    },
    salt: {
      total: 0.15,
    },
  },
  {
    id: 15,
    name: "Hovis Seed Sensations Seven Seeds Medium Sliced Seeded Bread",
    manufacturer: "Hovis",
    energy: {
      kcal: 276.0,
      kj: 1171.0,
    },
    carbohydrates: {
      total: 42.0,
      fiber: 5.3,
      sugar: 3.8,
      addedSugar: 0.0,
      starch: 0.0,
    },
    fat: {
      total: 6.6,
      saturated: 0.6,
      monounsaturated: 0.0,
      trans: 0.0,
    },
    protein: {
      total: 10.0,
    },
    salt: {
      total: 0.88,
    },
  },
  {
    id: 17,
    name: "Lurpak Butter, Slightly Salted",
    manufacturer: "Lurpak",
    energy: {
      kcal: 739.0,
      kj: 3038.0,
    },
    carbohydrates: {
      total: 0.7,
      fiber: 0.0,
      sugar: 0.7,
      addedSugar: 0.0,
      starch: 0.0,
    },
    fat: {
      total: 82.0,
      saturated: 52.0,
      monounsaturated: 0.0,
      trans: 0.0,
    },
    protein: {
      total: 0.6,
    },
    salt: {
      total: 1.2,
    },
  },
  {
    id: 100,
    name: "Saint Agur Blue Cheese",
    manufacturer: "Sain Agur",
    energy: {
      kcal: 361.0,
      kj: 1493.0,
    },
    carbohydrates: {
      total: 0.5,
      fiber: 0.0,
      sugar: 0.5,
      addedSugar: 0.0,
      starch: 0.0,
    },
    fat: {
      total: 33.0,
      saturated: 23.0,
      monounsaturated: 0.0,
      trans: 0.0,
    },
    protein: {
      total: 16.0,
    },
    salt: {
      total: 2.2,
    },
  },
];
export default data;
