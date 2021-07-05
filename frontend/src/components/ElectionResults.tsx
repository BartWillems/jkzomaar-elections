import React from "react";
import Typography from "@material-ui/core/Typography";
import ReconnectingWebSocket from "reconnecting-websocket";
import ApiClient from "../Api";

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

const ElectionResults = () => {
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
    <div style={{ width: "100%" }}>
      <Typography variant="h3" component="h3" gutterBottom>
        Voorzitter
      </Typography>
      <ul>
        {result.voorzitters.map((candidate, index) => (
          <li key={index}>
            {candidate.name} - {candidate.count}
          </li>
        ))}
      </ul>
      <Typography variant="h3" component="h3" gutterBottom>
        Ondervoorzitter
      </Typography>
      <ul>
        {result.ondervoorzitters.map((candidate, index) => (
          <li key={index}>
            {candidate.name} - {candidate.count}
          </li>
        ))}
      </ul>
      <Typography variant="h3" component="h3" gutterBottom>
        Penning Meester
      </Typography>
      <ul>
        {result.penningMeesters.map((candidate, index) => (
          <li key={index}>
            {candidate.name} - {candidate.count}
          </li>
        ))}
      </ul>
      <Typography variant="h3" component="h3" gutterBottom>
        Secretaris
      </Typography>
      <ul>
        {result.secretarissen.map((candidate, index) => (
          <li key={index}>
            {candidate.name} - {candidate.count}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default ElectionResults;
