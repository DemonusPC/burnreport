import { Box, Button, Form, FormField, Heading, TextInput } from 'grommet';
import React from 'react'
import { Redirect } from 'react-router-dom';
import { SearchSuggestion } from '../../../containers/ProductSearchForm';
import ReportForm from '../../../containers/ReportForm';
import SearchForm from '../../../containers/SearchForm';
import { Portion } from '../../../product/product';
import { ConsumedProduct, ConsumedRaw } from '../../../report/report';
import { getProductSearchSuggestions, postRecipie } from '../../../util/data/requests';

type IngredientCreateCommand = {
    amount: number;
    product_id: number;
}

export type RecipieCreateCommand = {
    name: string;
    ingredients: IngredientCreateCommand[]
}



const AddRecipie = () => {
    const [sent, setSent] = React.useState(false);
    const [ingredients, setIngredients] = React.useState<ConsumedProduct[]>([])

    const onIngredientsSet = (consumed: ConsumedProduct[]) => {
        setIngredients(consumed);
    }

    const send = (event: any) => {
        const payload: RecipieCreateCommand = {
            name: event.value.name,
            ingredients: ingredients.map((i) => ({ amount: i.amount, product_id: i.id }))
        }

        postRecipie(payload).then(() => setSent(true));
    }

    return (
        <Box pad="large" gridArea="main">
            <Box>
                <Heading>Add Recipie</Heading>
                <Box>
                    <Form onSubmit={send}>
                        <FormField
                            name={"name"}
                            label={"Recipie name"}
                            required={true}
                        >
                            <TextInput name={"name"} />
                        </FormField>

                        <ReportForm setReportFunction={() => console.log("")} onSend={onIngredientsSet} />
                    </Form>
                </Box>
                {sent && <Redirect to="/recipies" />}
            </Box>
        </Box>
    )
}

export default AddRecipie;