import { fireEvent, render } from "@testing-library/react"
import AddRecipie from "."
import userEvent from "@testing-library/user-event"
import { GetSearch } from "../../product/productApi";
import { SearchEntity } from "../../../containers/ProductSearchForm";
import { MemoryRouter, Route } from "react-router-dom";


const mockSearch: GetSearch = async () => {
    return [{ id: 1, text: "Cheese", entity: SearchEntity.Product }];
}

describe("Given I render the AddRecipie page", () => {
    describe("when I fill in the form and click 'submit", () => {
        it("should send a create recipie", async () => {
            const mockSend = jest.fn();

            const mockPromiseSend = async (data: unknown) => {
                mockSend(data);
            }

            const renderResult = render(
                <MemoryRouter initialEntries={['/recipie/add']}>
                    <Route path="/recipie/add">
                        <AddRecipie productSearch={mockSearch} addRecipie={mockPromiseSend} />
                    </Route>
                </MemoryRouter>
            );

            const { getByLabelText, findByLabelText, findByText, getByText } = renderResult;

            const nameInput = getByLabelText("recipie-name")

            await userEvent.type(nameInput, "Cool new recipie");

            const searchInput = getByLabelText("search");

            await userEvent.type(searchInput, "Chee");

            await userEvent.click(await findByText("Cheese"));

            expect(await findByLabelText("Cheese-amount-select")).toBeInTheDocument();

            const perInput = getByLabelText("Cheese-amount-select");
            await userEvent.clear(perInput);
            await userEvent.type(perInput, "50");

            await userEvent.click(getByText("Submit"));

            expect(mockSend).toHaveBeenCalledWith({
                name: "Cool new recipie", ingredients: [
                    {
                        amount: 50,
                        product_id: 1
                    }
                ]
            })

        })
    })
})