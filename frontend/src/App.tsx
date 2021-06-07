import React from "react";
import "@fontsource/roboto";
import "./App.css";
import Router from "./Router";
import { Container } from "@material-ui/core";
import { makeStyles } from "@material-ui/core/styles";

const useStyles = makeStyles((theme) => ({
  paper: {
    marginTop: theme.spacing(8),
    display: "flex",
    flexDirection: "column",
    alignItems: "center",
  },
}));

function App() {
  const classes = useStyles();
  return (
    <Container className={classes.paper}>
      <Router />
    </Container>
  );
}

export default App;
