import { Accordion, AccordionPanel, Anchor, Box, Button, Heading, PageHeader } from 'grommet';
import React from 'react'
import { useHistory, useParams } from 'react-router-dom';
import useSWR from 'swr';
import AdditionalTable from '../../../containers/AdditionalTable';
import Bar from '../../../containers/Bar';
import NutrientTable from '../../../containers/NutrientTable';
import { nutrientsToBarTotal, nutrientsToBarValues } from '../../../nutrients/nutrients';
import { vitaminsToRow } from '../../../nutrients/vitamins';
import { Recipie } from '../../../recipie/recipie';
import { deleteRecipie, fetcher } from '../../../util/data/requests';

interface IdParams {
    id: string;
}

const RecipieView = () => {
    const history = useHistory();
    const params: IdParams = useParams<IdParams>();
    const parsed = Number.parseInt(params.id);

    const { data, error } = useSWR<Recipie | null>(
        encodeURI(`/api/recipies/${parsed}`),
        fetcher
    );


    if (error) return <div>Error</div>;
    if (!data) return <div>loading...</div>;

    return (
        <Box pad="large" gridArea='main'>
            {/* <Heading level={2}>{data.name}</Heading> */}
            <PageHeader
                title={data.name}
                subtitle="Recipie"
            />
            <Bar data={data.total} mapToBarValues={nutrientsToBarValues} calculateTotal={nutrientsToBarTotal} />
            <NutrientTable
                nutrients={data.total}
                amount={100}
                baseUnit={1}
            />
            <Accordion animate={true} multiple={false}>
                <AccordionPanel label="Vitamins">
                    <AdditionalTable
                        entity={data.total.vitamins}
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
                        await deleteRecipie(data.id);
                        history.push("/products");
                    }}
                />
            </Box>
        </Box>
    )
}

export default RecipieView;