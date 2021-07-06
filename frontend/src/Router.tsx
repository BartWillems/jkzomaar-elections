import React from "react";
import { Switch, Route } from "react-router-dom";
import VoteBooth from "./components/VoteBooth";
import ElectionResults from "./components/ElectionResults";
import Admin from "./components/Admin";

export const Routes = {
  ElectionResults: "/",
  VoteBooth: "/votes/:id",
  Admin: "/secret",
};

const Router = () => {
  return (
    <Switch>
      <Route path={Routes.VoteBooth} exact>
        <VoteBooth />
      </Route>

      <Route path={Routes.ElectionResults} exact>
        <ElectionResults />
      </Route>

      <Route path={Routes.Admin} exact>
        <Admin />
      </Route>

      <Route path="*">
        <div>Page not found</div>
      </Route>
    </Switch>
  );
};

export default Router;
