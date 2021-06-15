import React from "react";
import "@fontsource/roboto";
import "./App.css";
import Router from "./Router";
import { Container } from "@material-ui/core";
import { makeStyles } from "@material-ui/core/styles";
import Typography from "@material-ui/core/Typography";

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
      <Typography variant="h1" component="h2" gutterBottom>
        JK Zomaar Verkiezingen 2021
      </Typography>
      <Router />
    </Container>
  );
}

export default App;
