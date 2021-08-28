import { Box, Button } from "grommet";
import styled from "styled-components";
import SearchForm from "../../containers/SearchForm"


// const darkbrick = "#b62203";
// const darkbrickhover = "#801100";

// #404345 

const darkbrick= "#404345";
const darkbrickhover = "#1a1b1c" ;

const BButton = styled(Button)`
    background-color: ${darkbrick};
    color: white;
    border-radius: 6px;
    box-shadow: 0 4px 0 ${darkbrickhover};
    border: none;

    &:hover {
        background-color: ${darkbrickhover};
        border: none;
        box-shadow: 0 4px 0 ${darkbrickhover};
    }
`;

const SButton = styled(Button)`
    background-color: transparent;
    color: black;
    border-radius: 6px;
    box-shadow: 0 4px 0 ${darkbrick};
    border: solid 1px ${darkbrick};

    &:hover {
        color: white;
        background-color: ${darkbrick};
        border: solid 1px ${darkbrick};
        box-shadow: 0 4px 0 ${darkbrick};
    }
`;

const TestPage = () => {
    return (
        <>
        <SearchForm selectedFunction={() => {}} suggestFunction={async (suggestion: string)=>{}} />
        <Box margin={"large"} direction="row" gap="medium">
            <Button type="submit" primary={true} label="Primary"/>
            <Button type="submit" label="Secondary" />
            <Button type="submit" label= "Test"/>
        </Box>
        <Box margin="large" direction="row" gap="medium">
            <BButton type="submit" primary={true} label="Primary"/>
            <SButton type="submit" label="Secondary" />
            <SButton type="submit" label= "Test"/>
        </Box>
        </>
    )
}

export default TestPage;