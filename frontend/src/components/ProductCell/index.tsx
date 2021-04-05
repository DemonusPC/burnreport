import React from 'react';
import { Link } from 'react-router-dom';
import { Product } from '../../util/schema/product';


const ProductCell = ({id, name}: Product) => {
    return (
        <Link to={encodeURI(`/products/${id}`)}>{name}</Link>
    )
} 

export default ProductCell;
