import { Accordion, AccordionPanel, Box, Button, PageHeader } from 'grommet';
import React from 'react'
import { useHistory, useParams } from 'react-router-dom';
import AdditionalTable from '../../../containers/AdditionalTable';
import Bar from '../../../containers/Bar';
import NutrientTable from '../../../containers/NutrientTable';
import { nutrientsToBarTotal, nutrientsToBarValues } from '../../../nutrients/nutrients';
import { vitaminsToRow } from '../../../nutrients/vitamins';
import { deleteRecipie } from '../../../util/data/requests';
import { GetRecipie, fetchRecipie } from './recipieApi';

interface IdParams {
    id: string;
}

type RecipieViewProps = {
    recipieFetcher?: GetRecipie;
}

const RecipieView = ({ recipieFetcher = fetchRecipie }: RecipieViewProps) => {
    const history = useHistory();
    const params: IdParams = useParams<IdParams>();
    const parsed = Number.parseInt(params.id);

    const { recipie, error } = recipieFetcher(parsed);



    if (error) return <div>Error</div>;
    if (!recipie) return <div>loading...</div>;

    return (
        <Box pad="large" gridArea='main'>
            <PageHeader
                title={recipie.name}
                subtitle="Recipie"
            />
            <Bar data={recipie.total} mapToBarValues={nutrientsToBarValues} calculateTotal={nutrientsToBarTotal} />
            <NutrientTable
                nutrients={recipie.total}
                amount={100}
                baseUnit={1}
            />
            <Accordion animate={true} multiple={false}>
                <AccordionPanel label="Vitamins">
                    <AdditionalTable
                        entity={recipie.total.vitamins}
                        mapper={vitaminsToRow}
                        unit={"mg"}
                    />
                </AccordionPanel>
            </Accordion>
            <Box
                justify="between"
                alignContent="center"
                direction="row"
                margin={{ top: "xlarge" }}
                gap="large"
            >
                <Button
                    fill={false}
                    color="status-critical"
                    type="button"
                    label="Delete Recipie"
                    onClick={async () => {
                        await deleteRecipie(recipie.id);
                        history.push("/products");
                    }}
                />
            </Box>
        </Box>
    )
}

export default RecipieView;