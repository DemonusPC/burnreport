import { Box, Heading, FileInput, Text, Form, FormField, TextInput, MaskedInput, Button } from "grommet";
import React from "react";
import { Redirect } from "react-router-dom";
import { postSpi } from "../../util/data/requests";

export type Spi = {
    numericCode: number;
    alphabeticCode: number;
    name: string;
}

const propertyToNumber = (property: number): number => {
    if (property) {
        if (!Number.isNaN(+property)) {
            return +property;
        }
    }
    return 0;
};

const toSpi = (event: any): Spi => {
    return {
        numericCode: propertyToNumber(event.numericCode),
        alphabeticCode: event.alphabeticCode,
        name: event.name
    }

}

const AddSpi = () => {
    const [sent, setSent] = React.useState(false);

    const onSubmit = (event: any) => {
        const spi = toSpi(event.value);
        postSpi(spi).then((result) => {
            setSent(true);
        });
    };
    return (
        <Box pad="large" gridArea="main">
            <Box>
                <Heading>Add Product</Heading>
                <Box>
                    <Form onSubmit={onSubmit}>

                        <FormField
                            name={"numericCode"}
                            label={"Numeric Code"}
                            required={true}
                        >
                            <MaskedInput name={"numericCode"} mask={[{
                                regexp: /^\d+$/,
                            },]} />
                        </FormField>
                        <FormField
                            name={"alphabeticCode"}
                            label={"Alphabetic Code"}
                            required={true}
                        >
                            <TextInput name={"alphabeticCode"} />
                        </FormField>
                        <FormField
                            name={"name"}
                            label={"Name"}
                            required={true}
                        >
                            <TextInput name={"name"} />
                        </FormField>

                        <Box direction="row" margin={{ top: "large" }} gap="medium">
                            <Button type="submit" primary label="Submit" />
                        </Box>
                    </Form>
                </Box>
                {sent && <Redirect to="/products" />}
            </Box>
        </Box>
    );
}

export default AddSpi