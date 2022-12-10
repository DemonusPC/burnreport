import { fireEvent, render } from "@testing-library/react"
import AddRecipie from "."


describe("Given I render the AddRecipie page", () => {
    describe("when I fill in the form and click 'submit", () => {
        it("should send a create recipie", async () => {
            const renderResult = render(
                <AddRecipie />
            );

            const { getByLabelText } = renderResult;

            const nameInput = getByLabelText("recipie-name")

            fireEvent.change(nameInput, { target: { value: 'Cool new recipie' } })


        })
    })
})