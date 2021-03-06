import React from "react";
import "@fontsource/roboto";
import "./App.css";
import Router from "./Router";
import { Container } from "@material-ui/core";
import { makeStyles } from "@material-ui/core/styles";
import Typography from "@material-ui/core/Typography";

const useStyles = makeStyles((theme) => ({
  paper: {
    maxWidth: 1024,
    margin: `${theme.spacing(1)}px auto`,
    padding: theme.spacing(2),
  },
}));

function App() {
  const classes = useStyles();
  return (
    <Container className={classes.paper}>
      <Typography variant="h2" component="h2" align="center">
        JK Zomaar
      </Typography>
      <Typography variant="h4" component="h4" gutterBottom align="center">
        Verkiezingen 2021
      </Typography>
      <Router />
    </Container>
  );
}

export default App;
