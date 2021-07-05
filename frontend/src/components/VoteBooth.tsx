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
import Grid from "@material-ui/core/Grid";

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

interface Candidate {
  name: string;
}

interface Candidates {
  voorzitters: Candidate[];
  ondervoorzitters: Candidate[];
  penningMeesters: Candidate[];
  secretarissen: Candidate[];
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

        setValidBallot(false);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [id]);

  return (
    <>
      {loading && <p>Loading...</p>}
      {ballot && <VoteForm ballot={ballot} />}
      {!validBallot && (
        <Alert severity="error">Foutief of reeds gebruikt stembiljet!</Alert>
      )}
    </>
  );
};

const VoteForm = ({ ballot }: { ballot: Ballot }) => {
  const [candidates, setCandidates] = React.useState<Candidates | null>(null);
  const [voorzitter, setVoorzitter] = React.useState<string | null>(null);
  const [ondervoorzitter, setOnderVoorzitter] =
    React.useState<string | null>(null);
  const [penningMeester, setPenningMeester] =
    React.useState<string | null>(null);
  const [secretaris, setSecretaris] = React.useState<string | null>(null);
  const [isValid, setIsValid] = React.useState(false);

  useEffect(() => {
    ApiClient.get("/candidates")
      .then((resp) => setCandidates(resp.data))
      .catch((error) => console.error(error));
  }, []);

  useEffect(() => {
    if (!voorzitter || !ondervoorzitter || !penningMeester || !secretaris) {
      return;
    }

    setIsValid(true);
  }, [voorzitter, ondervoorzitter, penningMeester, secretaris]);

  return (
    <Grid container spacing={2}>
      <Grid item sm={12} xs={12} md={12} lg={12} xl={12}>
        <Card>
          <CardContent>
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
                    value={candidate.name}
                    control={<Radio />}
                    label={candidate.name}
                    key={candidate.name}
                  />
                ))}
              </RadioGroup>
            </FormControl>
          </CardContent>
        </Card>
      </Grid>

      <Grid item sm={12} xs={12} md={12} lg={12} xl={12}>
        <Card>
          <CardContent>
            <FormControl component="fieldset">
              <FormLabel component="legend">Onder Voorzitter</FormLabel>
              <RadioGroup
                aria-label="gender"
                name="gender1"
                value={ondervoorzitter}
                onChange={(ev, value: string) => setOnderVoorzitter(value)}
              >
                {candidates?.ondervoorzitters.map((candidate) => (
                  <FormControlLabel
                    value={candidate.name}
                    control={<Radio />}
                    label={candidate.name}
                    key={candidate.name}
                  />
                ))}
              </RadioGroup>
            </FormControl>
          </CardContent>
        </Card>
      </Grid>

      <Grid item sm={12} xs={12} md={12} lg={12} xl={12}>
        <Card>
          <CardContent>
            <FormControl component="fieldset">
              <FormLabel component="legend">Penning Meester</FormLabel>
              <RadioGroup
                aria-label="gender"
                name="gender1"
                value={penningMeester}
                onChange={(ev, value: string) => setPenningMeester(value)}
              >
                {candidates?.penningMeesters.map((candidate) => (
                  <FormControlLabel
                    value={candidate.name}
                    control={<Radio />}
                    label={candidate.name}
                    key={candidate.name}
                  />
                ))}
              </RadioGroup>
            </FormControl>
          </CardContent>
        </Card>
      </Grid>

      <Grid item sm={12} xs={12} md={12} lg={12} xl={12}>
        <Card>
          <CardContent>
            <FormControl component="fieldset">
              <FormLabel component="legend">Secretaris</FormLabel>
              <RadioGroup
                aria-label="gender"
                name="gender1"
                value={secretaris}
                onChange={(ev, value: string) => setSecretaris(value)}
              >
                {candidates?.secretarissen.map((candidate) => (
                  <FormControlLabel
                    value={candidate.name}
                    control={<Radio />}
                    label={candidate.name}
                    key={candidate.name}
                  />
                ))}
              </RadioGroup>
            </FormControl>
          </CardContent>
        </Card>
      </Grid>
    </Grid>
  );
};

export default VoteBooth;
