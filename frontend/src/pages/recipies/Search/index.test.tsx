import { render } from "@testing-library/react";
import RecipieSearch from ".";

describe("Given a user navigates to the Recipie Search Page", () => {
    describe("When they search for a word", () => {
        it("they can select a value and see a list of all possible search values with links to recipies", async () => {
            const renderResult = render(<RecipieSearch />);

            const { findByText } = renderResult;

            expect(await findByText("Recipie Search")).toBeInTheDocument();

        })
    })


})