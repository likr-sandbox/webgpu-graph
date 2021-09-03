import React from "react";
import { BrowserRouter as Router, Route, Switch } from "react-router-dom";
import Root from "./pages/Root";

export default function App() {
  return (
    <Router>
      <Switch>
        <Route path="/" component={Root} exact />
      </Switch>
    </Router>
  );
}
