import React from 'react';
import './App.css';
import SideBar from "./sidebar/SideBar";
import routes from "./routes/routes";
import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link, Redirect
} from "react-router-dom";

function App() {
    return (
        <div className="App">
            <header className="App-header">
                <div
                    className="App-link"
                    rel="noopener noreferrer">
                    Dolly Peg Hall
                </div>
            </header>
            <Router>
                <SideBar pageWrapId={"page-wrap"} outerContainerId={"App"} routes={routes}/>
                <main id="page-wrap">
                    <Switch>
                        {routes.map((prop, key) => {
                            return (
                                <Route
                                    path={prop.path}
                                    component={prop.component}
                                    key={key}
                                />
                            );
                        })}
                    </Switch>
                </main>
            </Router>
        </div>
    );
}

export default App;
