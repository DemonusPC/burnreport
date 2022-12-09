import { Accordion, AccordionPanel, Anchor, Box, Heading, PageHeader } from 'grommet';
import { FormPrevious } from 'grommet-icons'
import React from 'react'
import { Link, useHistory, useParams } from 'react-router-dom';
import useSWR from 'swr';
import AnchorLink from '../../../components/AnchorLink';
import AdditionalTable from '../../../containers/AdditionalTable';
import Bar from '../../../containers/Bar';
import NutrientTable from '../../../containers/NutrientTable';
import { nutrientsToBarTotal, nutrientsToBarValues } from '../../../nutrients/nutrients';
import { vitaminsToRow } from '../../../nutrients/vitamins';
import { Recipie } from '../../../recipie/recipie';
import { fetcher } from '../../../util/data/requests';

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
        </Box>
    )
}

export default RecipieView;