import { RenderResult } from "@testing-library/react";
import { Nutrients } from "../../nutrients/nutrients";

export const nutrientsAreShown = (page: RenderResult, nutrients: Nutrients) => {

    const { getByText } = page;

    const { energy, carbohydrates, fat, protein, salt } = nutrients;

    const toFind = [
        { label: "Energy", value: `${energy.kj} kJ / ${energy.kcal} kcal` },
        { label: "carbohydrates", value: `${carbohydrates.total || 0} g` },
        { label: "sugar", value: `${carbohydrates.sugar || 0} g` },
        { label: "fiber", value: `${carbohydrates.fiber || 0} g` },
        { label: "addedSugar", value: `${carbohydrates.addedSugar || 0} g` },
        { label: "starch", value: `${carbohydrates.starch || 0} g` },
        { label: "fat", value: `${fat.total || 0} g` },
        { label: "saturated", value: `${fat.saturated || 0} g` },
        { label: "protein", value: `${protein.total} g` },
        { label: "salt", value: `${salt.total} g` },
    ];

    toFind.forEach(({ label, value }) => {
        expect(getByText(label)).toBeInTheDocument();
        expect(getByText(value)).toBeInTheDocument();
    });
}

test("Placeholder", () => expect(true).toEqual(true))
