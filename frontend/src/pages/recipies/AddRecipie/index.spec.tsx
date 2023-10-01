import { fireEvent, render } from "@testing-library/react"
import AddRecipie from "."
import userEvent from "@testing-library/user-event"


describe("Given I render the AddRecipie page", () => {
    describe("when I fill in the form and click 'submit", () => {
        it("should send a create recipie", async () => {
            const renderResult = render(
                <AddRecipie />
            );

            const { getByLabelText } = renderResult;

            const nameInput = getByLabelText("recipie-name")

            userEvent.type(nameInput, "Cool new recipie");

        })
    })
})