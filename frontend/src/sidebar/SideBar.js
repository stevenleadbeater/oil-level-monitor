import React from "react";
import {slide as Menu} from "react-burger-menu";
import './SideBar.css';
import {Link} from "react-router-dom";

export default props => {
    return (
        <Menu {...props}>
            {props.routes.reverse().map((prop, key) => {
                return (
                    <li key={key}>
                        <Link
                            to={prop.path}
                            className="menu-item">
                            <img src={prop.icon} className="menu-logo" alt={prop.name}/>
                            <span className="menu-text">{prop.name}</span>
                        </Link>
                    </li>
                );
            })}
        </Menu>
    );
};
