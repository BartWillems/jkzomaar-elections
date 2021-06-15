import React from "react";
import { Switch, Route } from "react-router-dom";
import VoteBooth from "./components/VoteBooth";
import ElectionResults from "./components/ElectionResults";

const Router = () => {
  return (
    <Switch>
      <Route path="/votes/:id" exact>
        <VoteBooth />
      </Route>

      <Route path="/" exact>
        <ElectionResults />
      </Route>

      <Route path="*">
        <div>Page not found</div>
      </Route>
    </Switch>
  );
};

export default Router;
