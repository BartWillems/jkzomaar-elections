import React from "react";
import Typography from "@material-ui/core/Typography";
import ReconnectingWebSocket from "reconnecting-websocket";
import { Theme, makeStyles } from "@material-ui/core/styles";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import clsx from "clsx";
import { Cell, Legend, PieChart, Pie, ResponsiveContainer } from "recharts";

import ApiClient from "../Api";

// 16 material olors as that is the maximum amount of beverages
// Consists of 4 complimenting groups
const COLORS = [
  //
  "#673ab7",
  "#009688",
  "#ffc107",
  "#607d8b",
  //
  "#9c27b0",
  "#00bcd4",
  "#ffeb3b",
  "#9e9e9e",
  //
  "#f44336",
  "#3f51b5",
  "#4caf50",
  "#ff9800",
  //
  "#e91e63",
  "#2196f3",
  "#8bc34a",
  "#ff5722",
];

interface Count {
  count: number;
  name: string;
}

interface Results {
  voorzitters: Count[];
  ondervoorzitters: Count[];
  penningMeesters: Count[];
  secretarissen: Count[];
}

const WebsocketURI =
  process.env.REACT_APP_WS_URL ||
  ((window.location.protocol === "https:" && "wss://") || "ws://") +
    window.location.host +
    "/ws";

const useStyles = makeStyles((theme: Theme) => ({
  paper: {
    padding: theme.spacing(2),
    display: "flex",
    overflow: "auto",
    flexDirection: "column",
  },
  fixedHeight: {
    height: 400,
  },
}));

const ElectionChart = ({
  count,
  offset,
}: {
  count: Count[];
  offset: number;
}) => {
  return (
    <ResponsiveContainer>
      <PieChart width={730} height={250}>
        <Pie
          data={count}
          dataKey="count"
          nameKey="name"
          outerRadius={"80%"}
          label
        >
          {count.map((entry, index) => (
            <Cell
              key={index}
              fill={COLORS[(index + offset * 4) % COLORS.length]}
            />
          ))}
        </Pie>
        <Legend verticalAlign="bottom" height={50} />
      </PieChart>
    </ResponsiveContainer>
  );
};

const ElectionResults = () => {
  const classes = useStyles();
  const fixedHeightPaper = clsx(classes.paper, classes.fixedHeight);

  const [result, setResult] = React.useState<Results>({
    voorzitters: [],
    ondervoorzitters: [],
    penningMeesters: [],
    secretarissen: [],
  });

  React.useEffect(() => {
    ApiClient.get(`/result`)
      .then((resp) => {
        setResult(resp.data);
      })
      .catch((error) => {
        console.error(error);
      });
  }, []);

  React.useEffect(() => {
    const rws = new ReconnectingWebSocket(`${WebsocketURI}`);

    rws.onmessage = (update) => {
      console.log(update.data);
      const res: Results = JSON.parse(update.data);

      setResult(res);
    };

    rws.onclose = (msg) => {
      console.log(msg);
      if (!msg.wasClean) {
        console.log("unclean websocket shutdown");
        // setConnected(false);
      }
    };

    rws.onerror = () => {
      // setConnected(false);
    };

    rws.onopen = () => {
      // setConnected(true);
    };

    return () => {
      rws.close(1000);
    };
  }, []);

  return (
    <Grid container spacing={3}>
      <Grid item xs={12} sm={12} md={6} lg={6} xl={6}>
        <Paper className={fixedHeightPaper}>
          <Typography style={{ textAlign: "center" }} variant="h4">
            Voorzitter
          </Typography>
          <ElectionChart count={result.voorzitters} offset={0} />
        </Paper>
      </Grid>

      <Grid item xs={12} sm={12} md={6} lg={6} xl={6}>
        <Paper className={fixedHeightPaper}>
          <Typography style={{ textAlign: "center" }} variant="h4">
            Ondervoorzitter
          </Typography>
          <ElectionChart count={result.ondervoorzitters} offset={1} />
        </Paper>
      </Grid>

      <Grid item xs={12} sm={12} md={6} lg={6} xl={6}>
        <Paper className={fixedHeightPaper}>
          <Typography style={{ textAlign: "center" }} variant="h4">
            Penningmeester
          </Typography>
          <ElectionChart count={result.penningMeesters} offset={2} />
        </Paper>
      </Grid>

      <Grid item xs={12} sm={12} md={6} lg={6} xl={6}>
        <Paper className={fixedHeightPaper}>
          <Typography style={{ textAlign: "center" }} variant="h4">
            Secretaris
          </Typography>
          <ElectionChart count={result.secretarissen} offset={3} />
        </Paper>
      </Grid>
    </Grid>
  );
};

export default ElectionResults;
