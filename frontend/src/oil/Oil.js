import React, {useState, useEffect} from "react";
import OilLevelAdapter from "../adapter/OilLevelAdapter";
import './Oil.css';
import {Chart} from 'react-charts'

export default () => {
    const [level, setLevel] = useState(0);
    const [levelHistory, setLevelHistory] = useState(null);

    const connection = new WebSocket(`ws://localhost:8120/ws/`);
    connection.onmessage = evt => {
        console.log(`Got message ${evt.data}`);
        const distance = JSON.parse(evt.data);
        setLevel(distance.distance);
    };

    setInterval(_ => {
        connection.send(Math.random())
    }, 4000);

    useEffect(() => {
        (async () => {
            const oilLevelAdapter = new OilLevelAdapter();

            const initialLevel = await oilLevelAdapter.get(1);
            setLevel(initialLevel.data.distance);

            const history = await oilLevelAdapter.getHistory(1);
            const data = history.data.map(item => [new Date(item.time_of_reading.secs_since_epoch * 1000), item.distance]);
            console.log(JSON.stringify(data));
            setLevelHistory([
                {
                    label: 'Oil Level',
                    data: data
                }
            ]);
        })();
    }, []);

    const axes = React.useMemo(
        () => [
            {primary: true, type: 'time', position: 'bottom'},
            {type: 'linear', position: 'left'}
        ],
        []
    );
    return (
        <div className="outer-container">
            <div className="container">
                <div className="bottom transformed circle"/>
                <div className="fill rectangle"/>
                <div className="level reading">{level}%</div>
                <div className="empty rectangle" style={{height: (100 - level) * 2 + 'px'}}/>
                <div className="fill transformed circle" style={{top: (100 - level) * 2 + 'px'}}/>
                <div className="top transformed circle"/>
            </div>
            {levelHistory !== null &&
            <div className="chart-container">
                <div className="chart">
                    <Chart data={levelHistory} axes={axes} tooltip dark/>
                </div>
            </div>
            }
        </div>
    );
}