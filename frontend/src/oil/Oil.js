import React, {useEffect, useState} from "react";
import OilLevelAdapter from "../adapter/OilLevelAdapter";
import * as THREE from 'three';
import './Oil.css';

export default () => {
    const [level, setLevel] = useState(0);

    const connection = new WebSocket(`ws://localhost:8120/ws/`);
    connection.onmessage = evt => {
        console.log(`Got message ${evt.data}`);
        const distance = JSON.parse(evt.data);
        setLevel(distance.distance);
    };

    setInterval(_ => {
        connection.send(Math.random())
    }, 4000);

    setTimeout(async _ => {

        const oilLevelAdapter = new OilLevelAdapter();
        const initialLevel = await oilLevelAdapter.get(1);

        setLevel(initialLevel.data.distance);

    });
    return (
        <div>
            <div className="container">
                <div className="bottom transformed circle"/>
                <div className="fill rectangle"/>
                <div className="level reading">{level}%</div>
                <div className="empty rectangle" style={{height : (100 - level) * 2 + 'px'}}/>
                <div className="fill transformed circle" style={{top : (100 - level) * 2 + 'px'}}/>
                <div className="top transformed circle"/>
            </div>
        </div>
    );
}