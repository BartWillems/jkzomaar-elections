import React, { useEffect } from "react";
import { useParams } from "react-router-dom";
import ApiClient from "../Api";
import Card from "@material-ui/core/Card";
import CardContent from "@material-ui/core/CardContent";
import Alert from "@material-ui/lab/Alert";
import Radio from "@material-ui/core/Radio";
import RadioGroup from "@material-ui/core/RadioGroup";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import FormControl from "@material-ui/core/FormControl";
import FormLabel from "@material-ui/core/FormLabel";

interface Ballot {
  id: string;
}

interface Vote {
  ballotId: string;
  voorzitter: string;
  ondervoorzitter: string;
  penningMeester: string;
  secretaris: string;
}

interface Candidates {
  voorzitters: string[];
  ondervoorzitters: string[];
  penningMeesters: string[];
  secretarissen: string[];
}

const VoteBooth = () => {
  const { id } = useParams<{ id: string }>();
  const [ballot, setBallot] = React.useState<Ballot>();
  const [loading, setLoading] = React.useState(true);
  const [validBallot, setValidBallot] = React.useState(true);

  React.useEffect(() => {
    ApiClient.get(`/ballots/${id}`)
      .then((resp) => {
        setBallot(resp.data);
      })
      .catch((error) => {
        console.error(error);
        if (error.response.status >= 400 && error.response.status < 500) {
          setValidBallot(false);
        }
      })
      .finally(() => {
        setLoading(false);
      });
  }, [id]);

  return (
    <Card>
      <CardContent>
        {loading && <p>Loading...</p>}
        {ballot && <VoteForm ballot={ballot} />}
        {!validBallot && (
          <Alert severity="error">Foutief of reeds gebruikt stembiljet!</Alert>
        )}
      </CardContent>
    </Card>
  );
};

const VoteForm = ({ ballot }: { ballot: Ballot }) => {
  const [candidates, setCandidates] = React.useState<Candidates | null>(null);
  const [voorzitter, setVoorzitter] = React.useState<string | null>(null);
  const [ondervoorzitter, setOnderVoorzitter] =
    React.useState<string | null>(null);
  const [isValid, setIsValid] = React.useState(false);

  useEffect(() => {
    if (!voorzitter) {
      return;
    }

    if (!ondervoorzitter) {
      return;
    }

    setIsValid(true);
  }, [voorzitter, ondervoorzitter]);

  return (
    <FormControl component="fieldset">
      <FormLabel component="legend">Voorzitter</FormLabel>
      <RadioGroup
        aria-label="gender"
        name="gender1"
        value={voorzitter}
        onChange={(ev, value: string) => setVoorzitter(value)}
      >
        {candidates?.voorzitters.map((candidate) => (
          <FormControlLabel
            value={candidate}
            control={<Radio />}
            label={candidate}
            key={candidate}
          />
        ))}
      </RadioGroup>
      <FormLabel component="legend">Onder Voorzitter</FormLabel>
      <RadioGroup
        aria-label="gender"
        name="gender1"
        value={ondervoorzitter}
        onChange={(ev, value: string) => setOnderVoorzitter(value)}
      >
        {candidates?.ondervoorzitters.map((candidate) => (
          <FormControlLabel
            value={candidate}
            control={<Radio />}
            label={candidate}
            key={candidate}
          />
        ))}
      </RadioGroup>
    </FormControl>
  );
};

export default VoteBooth;
